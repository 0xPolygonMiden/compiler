use core::fmt;

use cranelift_entity::entity_impl;
use intrusive_collections::{intrusive_adapter, LinkedList, LinkedListLink, UnsafeRef};

use crate::{diagnostics::Spanned, formatter::PrettyPrint, *};

intrusive_adapter!(pub RegionListAdapter = Box<Region>: Region { link: LinkedListLink });

/// A type alias for `LinkedList<RegionListAdapter>`
pub type RegionList = LinkedList<RegionListAdapter>;

/// A handle that refers to a [Region]
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegionId(u32);
entity_impl!(RegionId, "region");

#[derive(Spanned)]
pub struct Region {
    link: LinkedListLink,
    #[span]
    pub span: SourceSpan,
    pub id: RegionId,
    /// The block to which control is transferred on exit from this region
    ///
    /// The exit block must have a parameter list that matches the result types of the region,
    /// i.e. the results will be supplied as arguments to the exit block's parameter list
    ///
    /// This block is not part of the region itself, but instead represents something like a
    /// continuation, which tells us how control should be transferred out of the region.
    pub exit: Block,
    /// The arguments expected by this region
    pub params: Vec<Type>,
    /// The results returned from this region
    pub results: Vec<Type>,
    /// The set of blocks in this region, in layout order
    pub blocks: BlockList,
}
impl Region {
    pub(crate) fn new(
        span: SourceSpan,
        id: RegionId,
        entry: UnsafeRef<BlockData>,
        exit: Block,
    ) -> Self {
        let mut blocks = LinkedList::default();
        blocks.push_back(entry);
        Self {
            link: Default::default(),
            span,
            id,
            exit,
            params: vec![],
            results: vec![],
            blocks,
        }
    }

    pub(crate) fn new_empty(span: SourceSpan, id: RegionId) -> Self {
        Self {
            link: Default::default(),
            span,
            id,
            exit: Default::default(),
            params: vec![],
            results: vec![],
            blocks: LinkedList::default(),
        }
    }

    pub fn param(&self, index: usize) -> &Type {
        &self.params[index]
    }

    pub fn result(&self, index: usize) -> &Type {
        &self.results[index]
    }

    pub fn has_params(&self) -> bool {
        !self.params.is_empty()
    }

    pub fn has_results(&self) -> bool {
        !self.results.is_empty()
    }

    pub fn blocks(&self) -> &BlockList {
        &self.blocks
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    pub fn len(&self) -> usize {
        self.blocks.iter().count()
    }

    #[inline(always)]
    pub fn entry_block(&self) -> &BlockData {
        self.blocks.front().get().unwrap()
    }

    pub fn last_block(&self) -> &BlockData {
        self.blocks.back().get().unwrap()
    }

    pub fn insert_after(&mut self, block: UnsafeRef<BlockData>, after: Block) {
        let mut cursor = self.blocks.front_mut();
        while let Some(blk) = cursor.get() {
            if blk.id == after {
                cursor.insert_after(block);
                return;
            }
        }

        panic!(
            "unable to insert {} after {after}: {after} is not linked to the same region",
            block.id
        );
    }

    pub fn display<'a>(
        &'a self,
        current_function: FunctionIdent,
        dfg: &'a DataFlowGraph,
    ) -> impl fmt::Display + 'a {
        DisplayRegion {
            region: self,
            current_function,
            dfg,
        }
    }

    pub fn pretty_print<'a>(
        &'a self,
        current_function: FunctionIdent,
        dfg: &'a DataFlowGraph,
    ) -> impl PrettyPrint + 'a {
        DisplayRegion {
            region: self,
            current_function,
            dfg,
        }
    }
}

impl fmt::Debug for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Region")
            .field("id", &self.id)
            .field("params", &self.params)
            .field("results", &self.results)
            .field_with("blocks", |f| {
                f.debug_list().entries(self.blocks.iter().map(|blk| blk.id)).finish()
            })
            .finish()
    }
}

impl Eq for Region {}
impl PartialEq for Region {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.params == other.params
            && self.results == other.results
            && self
                .blocks
                .iter()
                .map(|blk| blk.id)
                .cmp(other.blocks.iter().map(|blk| blk.id))
                .is_eq()
    }
}

struct DisplayRegion<'a> {
    current_function: FunctionIdent,
    region: &'a Region,
    dfg: &'a DataFlowGraph,
}

impl<'a> fmt::Display for DisplayRegion<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pretty_print(f)
    }
}

impl<'a> formatter::PrettyPrint for DisplayRegion<'a> {
    fn render(&self) -> formatter::Document {
        use crate::formatter::*;

        let header =
            const_text("(") + const_text("region") + const_text(" ") + display(self.region.id);

        let params = if self.region.params.is_empty() {
            Document::Empty
        } else {
            self.region
                .params
                .iter()
                .fold(const_text("(") + const_text("params"), |acc, param| {
                    acc + const_text(" ") + text(format!("{param}"))
                })
                + const_text(")")
        };

        let params_and_results = if self.region.results.is_empty() {
            params
        } else {
            let open = const_text("(") + const_text("results");
            let results = self
                .region
                .results
                .iter()
                .fold(open, |acc, ty| acc + const_text(" ") + text(format!("{ty}")))
                + const_text(")");
            if matches!(params, Document::Empty) {
                results
            } else {
                params + const_text(" ") + results
            }
        };
        let params_and_results_indented = indent(6, nl() + params_and_results.clone());

        let signature = (const_text(" ") + params_and_results) | params_and_results_indented;

        let body = self.region.blocks.iter().fold(nl(), |acc, block_data| {
            let open = const_text("(")
                + const_text("block")
                + const_text(" ")
                + display(block_data.id.as_u32());

            let params = block_data
                .params(&self.dfg.value_lists)
                .iter()
                .map(|value| {
                    let ty = self.dfg.value_type(*value);
                    const_text("(")
                        + const_text("param")
                        + const_text(" ")
                        + display(*value)
                        + const_text(" ")
                        + text(format!("{ty}"))
                        + const_text(")")
                })
                .collect::<Vec<_>>();

            let params_singleline = params
                .iter()
                .cloned()
                .reduce(|acc, e| acc + const_text(" ") + e)
                .map(|params| const_text(" ") + params)
                .unwrap_or(Document::Empty);
            let params_multiline = params
                .into_iter()
                .reduce(|acc, e| acc + nl() + e)
                .map(|doc| indent(8, nl() + doc))
                .unwrap_or(Document::Empty);
            let header = open + (params_singleline | params_multiline);

            let body = indent(
                4,
                block_data
                    .insts()
                    .map(|inst| {
                        let inst_printer = crate::instruction::InstPrettyPrinter {
                            current_function: self.current_function,
                            id: inst,
                            dfg: &self.dfg,
                        };
                        inst_printer.render()
                    })
                    .reduce(|acc, doc| acc + nl() + doc)
                    .map(|body| nl() + body)
                    .unwrap_or_default(),
            );

            if matches!(acc, Document::Newline) {
                indent(4, acc + header + body + const_text(")"))
            } else {
                acc + nl() + indent(4, nl() + header + body + const_text(")"))
            }
        });

        header + signature + body + nl() + const_text(")")
    }
}
