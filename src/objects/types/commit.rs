use super::protocol::ObjectProtocol;
use super::protocol::SerdeValue;
use super::repo::RepoRef;
use super::IntegerObject;
use super::ListObject;
use super::SignatureObject;
use super::TimestampObject;
use crate::impl_object;
use crate::Result;
use gitrevset::dag::ops::PrefixLookup;
use std::fmt;

#[derive(Clone)]
pub struct CommitObject {
    repo_ref: RepoRef,
    commit_oid: git2::Oid,
}

impl CommitObject {
    pub fn new(repo_ref: RepoRef, commit_oid: git2::Oid) -> Self {
        Self {
            repo_ref,
            commit_oid,
        }
    }

    fn git_repo(&self) -> &git2::Repository {
        self.repo_ref.as_ref()
    }

    pub fn git_commit(&self) -> Result<git2::Commit> {
        Ok(self.git_repo().find_commit(self.commit_oid)?)
    }

    pub fn oid(&self) -> git2::Oid {
        self.commit_oid
    }
}

impl fmt::Display for CommitObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        hex::encode(&self.commit_oid.as_bytes()).fmt(f)
    }
}

impl_object! {
    impl CommitObject {
        pub fn author(&self) -> Result<SignatureObject> {
            Ok(self.git_commit()?.author().into())
        }

        pub fn user(&self) -> Result<SignatureObject> {
            self.author()
        }

        pub fn committer(&self) -> Result<SignatureObject> {
            Ok(self.git_commit()?.committer().into())
        }

        pub fn timestamp(&self) -> Result<TimestampObject> {
            Ok(self.git_commit()?.author().when().into())
        }

        pub fn time(&self) -> Result<TimestampObject> {
            self.timestamp()
        }

        pub fn date(&self) -> Result<TimestampObject> {
            self.timestamp()
        }

        pub fn message(&self) -> Result<String> {
            Ok(String::from_utf8_lossy(self.git_commit()?.message_bytes()).to_string())
        }

        pub fn desc(&self) -> Result<String> {
            self.message()
        }

        pub fn node(&self) -> String {
            hex::encode(self.commit_oid.as_bytes())
        }

        pub fn hash(&self) -> String {
            self.node()
        }

        pub fn short(&self) -> String {
            hex::encode(&self.commit_oid.as_bytes()[..6])
        }

        pub fn shortest(&self, min: Option<&IntegerObject>) -> Result<String> {
            let repo = self.repo_ref.gitrevset_repo()?;
            let mut hex = hex::encode(self.commit_oid.as_bytes());
            let mut min = min.map(|i| i.to_i64()).unwrap_or_default().min(1) as usize;
            for len in min..hex.len() {
                if repo.dag().vertexes_by_hex_prefix(&hex.as_bytes()[..len], 2)?.len() >= 2 {
                    min = len + 1;
                } else {
                    break;
                }
            }
            hex.truncate(min);
            Ok(hex)
        }

        pub fn parents(&self) -> Result<ListObject> {
            let commit = self.git_commit()?;
            let parents = commit
                .parent_ids()
                .map(|oid| Self::new(self.repo_ref.clone(), oid).to_object())
                .collect::<Vec<_>>();
            Ok(ListObject::from_vec(parents))
        }

        fn to_serde_value(&self) -> Result<SerdeValue> {
            let commit = self.git_commit()?;
            let hex = |i: git2::Oid| -> String { ::hex::encode(i.as_bytes()) };
            let utf8 = |b: &[u8]| -> String { String::from_utf8_lossy(b).to_string() };
            let value = serde_json::json!({
                "hash": hex(commit.id()),
                "parents": commit.parent_ids().map(|id| hex(id)).collect::<Vec<String>>(),
                "message": utf8(commit.message_bytes()),
                "author": self.author()?.to_serde_value()?,
                "committer": self.committer()?.to_serde_value()?,
            });
            Ok(value.into())
        }
    }
}

/// ASCII hex -> Binary hex.
fn normalize_hex(s: &str) -> Option<Vec<u8>> {
    let mut result = Vec::with_capacity(s.len());
    for &b in s.as_bytes() {
        match b {
            b'0'..=b'9' => result.push(b),
            b'a'..=b'f' => result.push(b),
            b'A'..=b'F' => result.push(b - b'A' + b'a'),
            _ => return None,
        }
    }
    Some(result)
}
