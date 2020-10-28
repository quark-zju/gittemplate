use super::protocol::Object;
use super::protocol::ObjectProtocol;
use super::protocol::SerdeValue;
use super::repo::RepoRef;
use super::CommitObject;
use super::GraphObject;
use super::ListObject;
use crate::impl_object;
use crate::Error;
use crate::Result;
use gitrevset::dag::Set;
use gitrevset::ext::OidExt;
use gitrevset::ext::VertexExt;
use std::fmt;

#[derive(Clone)]
pub struct RevsetObject {
    repo_ref: RepoRef,
    set: Set,
    reversed: bool,
}

impl RevsetObject {
    pub fn new(repo_ref: RepoRef, set: Set) -> Self {
        Self {
            repo_ref,
            set,
            reversed: false,
        }
    }

    fn git_repo(&self) -> &git2::Repository {
        self.repo_ref.as_ref()
    }

    pub fn set(&self) -> &Set {
        &self.set
    }

    fn commit_object(&self, oid: git2::Oid) -> CommitObject {
        let repo_ref = self.repo_ref.clone();
        CommitObject::new(repo_ref.clone(), oid)
    }
}

impl fmt::Display for RevsetObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.set.fmt(f)
    }
}

impl_object! {
    impl RevsetObject {
        pub fn count(&self) -> Result<i64> {
            Ok(self.set.count()? as i64)
        }

        pub fn commits(&self) -> Result<ListObject> {
            let set_iter = match self.reversed {
                false => self.set.iter()?,
                true => self.set.iter_rev()?,
            };
            let repo_ref = self.repo_ref.clone();
            let commit_iter = set_iter.map(move |v| match v {
                Err(e) => Err(Error::from(e)),
                Ok(v) => {
                    let oid = v.to_oid()?;
                    Ok(CommitObject::new(repo_ref.clone(), oid).to_object())
                },
            });
            Ok(ListObject::from_iter(commit_iter).with_separator("\n".to_string()))
        }

        pub fn contains(&self, commit: &CommitObject) -> Result<bool> {
            let oid = commit.oid();
            Ok(self.set.contains(&oid.to_vertex())?)
        }

        pub fn reverse(&self) -> Self {
            let mut other = self.clone();
            other.reversed = !other.reversed;
            other
        }

        pub fn graph(&self) -> GraphObject {
            GraphObject::new(self.repo_ref.clone(), self.set.clone())
        }

        fn deref_object(&self, name: &str) -> Result<Option<Object>> {
            // Do not deref to string.
            if ListObject::static_list_attrs().contains(&name) {
                Ok(Some(self.commits()?.to_object()))
            } else {
                Ok(None)
            }
        }

        fn to_serde_value(&self) -> Result<SerdeValue> {
            self.commits()?.to_serde_value()
        }
    }
}
