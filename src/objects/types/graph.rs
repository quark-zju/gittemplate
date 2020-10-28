use super::super::auto_deref;
use super::protocol::fmt;
use super::protocol::Object;
use super::protocol::ObjectProtocol;
use super::protocol::SerdeValue;
use super::repo::RepoRef;
use super::CommitObject;
use super::IntegerObject;
use super::LambdaObject;
use super::TimestampObject;
use crate::impl_object;
use crate::Result;
use dag::render::Ancestor;
use dag::render::Renderer;
use gitrevset::dag;
use gitrevset::dag::Set;
use gitrevset::ext::VertexExt;
use std::sync::Arc;

#[derive(Clone)]
pub struct GraphObject {
    git_repo: RepoRef,
    set: Set,
    func: Arc<dyn Fn(Object) -> Result<Object>>,
    min_row_height: usize,
}

impl GraphObject {
    pub fn new(git_repo: RepoRef, set: Set) -> Self {
        Self {
            git_repo,
            set,
            func: Arc::new(|o| Ok(o)),
            min_row_height: 3,
        }
    }

    fn commit_object(&self, oid: git2::Oid) -> CommitObject {
        let repo_ref = self.git_repo.clone();
        CommitObject::new(repo_ref.clone(), oid)
    }
}

impl fmt::Fmt for GraphObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut renderer = dag::render::GraphRowRenderer::new()
            .output()
            .with_min_row_height(self.min_row_height)
            .build_box_drawing();
        let iter = self.set.iter()?;
        let dag = self.set.dag().unwrap();
        for vertex in iter {
            let vertex = vertex?;
            let mut edges = Vec::new();
            let mut has_anonymous_parent = false;
            for p in dag.parent_names(vertex.clone())? {
                if self.set.contains(&p)? {
                    edges.push(Ancestor::Parent(p));
                } else {
                    let ancestors = dag.ancestors(p.clone().into())?;
                    let heads = dag.heads_ancestors(ancestors & self.set.clone())?;
                    if heads.is_empty()? {
                        has_anonymous_parent = true;
                    } else {
                        for h in heads.iter()? {
                            edges.push(Ancestor::Ancestor(h?));
                        }
                    }
                }
            }
            if has_anonymous_parent {
                edges.push(Ancestor::Anonymous);
            }
            let commit = CommitObject::new(self.git_repo.clone(), vertex.to_oid()?);
            let graph_width = renderer.width(Some(&vertex), Some(&edges)) as _;
            let graph_commit = GraphCommitObject {
                commit,
                graph_width,
            };
            let message = (self.func)(graph_commit.to_object())?.to_plain_string()?;
            let glyph = "o".to_string();
            let row = renderer.next_row(vertex, edges, glyph, message);
            let row = row.trim_end();
            f.write_str(&row)?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl_object! {
    impl GraphObject {
        pub fn map(&self, func: &LambdaObject) -> Self {
            let orig_func = self.func.clone();
            let new_func = func.clone();
            let chained_func = move |obj: Object| -> Result<Object> {
                let obj = orig_func(obj)?;
                new_func.apply(obj)
            };
            let mut other = self.clone();
            other.func = Arc::new(chained_func);
            other
        }

        pub fn shorten(&self, height: Option<&IntegerObject>) -> Result<Self> {
            let height = height.map(|i| i.to_i64()).unwrap_or_default().max(1);
            let mut other = self.clone();
            other.min_row_height = height as _;
            Ok(other)
        }
    }
}

pub struct GraphCommitObject {
    commit: CommitObject,
    graph_width: i64,
}

impl fmt::Fmt for GraphCommitObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // By default, print something more useful than just commit hash.
        // Consider using config override?
        let oid = self.commit.oid();
        f.write_str(&hex::encode(&oid.as_bytes()[0..6]))?;
        let commit = self.commit.git_commit()?;
        let author = commit.author();
        let name = author.name().unwrap_or_default();
        if !name.is_empty() {
            f.write_str("  ")?;
            f.write_str(name)?;
        }
        let date = author.when();
        let date = TimestampObject::from(date);
        let time = date.relative()?;
        if !time.is_empty() {
            f.write_str("  ")?;
            f.write_str(&time)?;
        }
        let message = commit
            .message()
            .unwrap_or_default()
            .split('\n')
            .next()
            .unwrap_or_default();
        if !message.is_empty() {
            f.write_str("\n")?;
            f.write_str(message)?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl_object! {
    impl GraphCommitObject {
        pub fn graphwidth(&self) -> i64 {
            self.graph_width
        }

        fn deref_object(&self, name: &str) -> Result<Option<Object>> {
            if CommitObject::static_list_attrs().contains(&name) {
                Ok(Some(self.commit.clone().to_object()))
            } else {
                auto_deref::default_deref_object(self, name)
            }
        }

        fn to_serde_value(&self) -> Result<SerdeValue> {
            self.commit.to_serde_value()
        }
    }
}
