use super::protocol::FromObjectRef;
use super::protocol::IntoObject;
use super::protocol::Object;
use super::protocol::ObjectProtocol;
use super::protocol::SerdeValue;
use super::CommitObject;
use super::RevsetObject;
use super::StringObject;
use crate::ast::Expr;
use crate::impl_object;
use crate::Result;
use once_cell::sync::OnceCell;
use serde_json::json;
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct RepoObject {
    repo_ref: RepoRef,
}
#[derive(Clone)]
pub struct RepoRef(Arc<RepoRefInner>);

struct RepoRefInner {
    inner: Arc<git2::Repository>,
    gitrevset_repo: OnceCell<gitrevset::Repo>,
}

impl From<git2::Repository> for RepoRef {
    fn from(git_repo: git2::Repository) -> Self {
        RepoRef(Arc::new(RepoRefInner {
            inner: Arc::new(git_repo),
            gitrevset_repo: Default::default(),
        }))
    }
}

impl AsRef<git2::Repository> for RepoRef {
    fn as_ref(&self) -> &git2::Repository {
        self.0.inner.as_ref()
    }
}

impl RepoRef {
    pub fn gitrevset_repo(&self) -> Result<&gitrevset::Repo> {
        self.0.gitrevset_repo.get_or_try_init(|| {
            let git_repo = self.0.inner.clone();
            Ok(gitrevset::Repo::open_from_repo(git_repo)?)
        })
    }

    pub fn git_repo(&self) -> &git2::Repository {
        &self.0.inner
    }
}

impl fmt::Display for RepoObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.git_repo().path().display().fmt(f)
    }
}

impl From<git2::Repository> for RepoObject {
    fn from(git_repo: git2::Repository) -> Self {
        RepoObject::new(RepoRef::from(git_repo))
    }
}

impl RepoObject {
    pub fn new(repo_ref: RepoRef) -> Self {
        Self { repo_ref }
    }
}

impl Deref for RepoObject {
    type Target = RepoRef;

    fn deref(&self) -> &Self::Target {
        &self.repo_ref
    }
}

impl RepoObject {
    /// Get a commit object by `oid`.
    pub fn get_commit(&self, oid: git2::Oid) -> CommitObject {
        CommitObject::new(self.repo_ref.clone(), oid)
    }
}

impl_object! {
    impl RepoObject {
        pub fn head(&self) -> Result<CommitObject> {
            let oid = self.git_repo().head()?.peel_to_commit()?.id();
            Ok(self.get_commit(oid))
        }

        pub fn config(&self, name: &StringObject) -> Result<String> {
            Ok(self.git_repo().config()?.get_string(name.as_ref())?)
        }

        pub fn lookup(&self, name: &StringObject) -> Result<CommitObject> {
            let repo = self.git_repo();
            let oid = repo.refname_to_id(name.as_ref())?;
            Ok(self.get_commit(oid))
        }

        pub fn root(&self) -> String {
            self.git_repo().path().display().to_string()
        }

        pub fn revs(&self, revset: Expr) -> Result<RevsetObject> {
            let repo = self.gitrevset_repo()?;
            let set = loop {
                if let Expr::Inlined(s) = &revset {
                    if let Ok(s) = StringObject::from_object(s) {
                        break repo.anyrevs(s.as_ref())?;
                    }
                }
                // Try to convert AST to revset AST.
                let expr = revset.into_gitrevset_expr()?;
                break repo.anyrevs(expr)?;
            };
            Ok(RevsetObject::new(self.repo_ref.clone(), set))
        }

        fn to_serde_value(&self) -> Result<SerdeValue> {
            let value = json!({
                "path": self.git_repo().path().display().to_string(),
            });
            Ok(value.into())
        }
    }
}

impl IntoObject for git2::Repository {
    fn into_object(self) -> Object {
        RepoObject::from(self).to_object()
    }
}
