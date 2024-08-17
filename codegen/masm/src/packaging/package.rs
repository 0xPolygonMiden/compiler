use alloc::{collections::BTreeSet, fmt, sync::Arc};

use miden_processor::Digest;
use midenc_hir::{formatter::DisplayHex, ConstantData, FunctionIdent, Ident, Signature, Symbol};
use midenc_session::{diagnostics::Report, Emit, LinkLibrary, Session};
use serde::{Deserialize, Serialize};

use super::{de, se};
use crate::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Package {
    /// Name of the package
    pub name: Symbol,
    /// Content digest of the package
    #[serde(
        serialize_with = "se::serialize_digest",
        deserialize_with = "de::deserialize_digest"
    )]
    pub digest: Digest,
    /// The package type and MAST
    #[serde(
        serialize_with = "se::serialize_mast",
        deserialize_with = "de::deserialize_mast"
    )]
    pub mast: MastArtifact,
    /// The rodata segments required by the code in this package
    pub rodata: Vec<Rodata>,
    /// The package manifest, containing the set of exported procedures and their signatures,
    /// if known.
    pub manifest: PackageManifest,
}
impl fmt::Debug for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Package")
            .field("name", &self.name)
            .field("digest", &format_args!("{}", DisplayHex::new(&self.digest.as_bytes())))
            .field_with("rodata", |f| f.debug_list().entries(self.rodata.iter()).finish())
            .field("manifest", &self.manifest)
            .finish_non_exhaustive()
    }
}
impl Emit for Package {
    fn name(&self) -> Option<Symbol> {
        Some(self.name)
    }

    fn output_type(&self, mode: midenc_session::OutputMode) -> midenc_session::OutputType {
        use midenc_session::OutputMode;
        match mode {
            OutputMode::Text => self.mast.output_type(mode),
            OutputMode::Binary => midenc_session::OutputType::Masp,
        }
    }

    fn write_to<W: std::io::Write>(
        &self,
        mut writer: W,
        mode: midenc_session::OutputMode,
        session: &Session,
    ) -> std::io::Result<()> {
        use midenc_session::OutputMode;
        match mode {
            OutputMode::Text => self.mast.write_to(writer, mode, session),
            OutputMode::Binary => {
                // Write magic
                writer.write_all(b"MASP\0")?;
                // Write format version
                writer.write_all(b"1.0\0")?;
                let data = bitcode::serialize(self).map_err(std::io::Error::other)?;
                writer.write_all(data.as_slice())
            }
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct PackageManifest {
    /// The set of exports in this package.
    pub exports: BTreeSet<PackageExport>,
    /// The libraries linked against by this package, which must be provided when executing the
    /// program.
    pub link_libraries: Vec<LinkLibrary>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageExport {
    pub id: FunctionIdent,
    #[serde(
        serialize_with = "se::serialize_digest",
        deserialize_with = "de::deserialize_digest"
    )]
    pub digest: Digest,
    /// We don't always have a type signature for an export
    #[serde(default)]
    pub signature: Option<Signature>,
}
impl fmt::Debug for PackageExport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PackageExport")
            .field("id", &format_args!("{}", self.id.display()))
            .field("digest", &format_args!("{}", DisplayHex::new(&self.digest.as_bytes())))
            .field("signature", &self.signature)
            .finish()
    }
}
impl PartialOrd for PackageExport {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for PackageExport {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.id.cmp(&other.id).then_with(|| self.digest.cmp(&other.digest))
    }
}

/// Represents a read-only data segment, combined with its content digest
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rodata {
    /// The content digest computed for `data`
    #[serde(
        serialize_with = "se::serialize_digest",
        deserialize_with = "de::deserialize_digest"
    )]
    pub digest: Digest,
    /// The address at which the data for this segment begins
    pub start: NativePtr,
    /// The raw binary data for this segment
    pub data: Arc<ConstantData>,
}
impl fmt::Debug for Rodata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rodata")
            .field("digest", &format_args!("{}", DisplayHex::new(&self.digest.as_bytes())))
            .field("start", &self.start)
            .field_with("data", |f| {
                f.debug_struct("ConstantData")
                    .field("len", &self.data.len())
                    .finish_non_exhaustive()
            })
            .finish()
    }
}
impl Rodata {
    pub fn size_in_bytes(&self) -> usize {
        self.data.len()
    }

    pub fn size_in_felts(&self) -> usize {
        self.data.len().next_multiple_of(4) / 4
    }

    pub fn size_in_words(&self) -> usize {
        self.size_in_felts().next_multiple_of(4) / 4
    }

    /// Attempt to convert this rodata object to its equivalent representation in felts
    ///
    /// The resulting felts will be in padded out to the nearest number of words, i.e. if the data
    /// only takes up 3 felts worth of bytes, then the resulting `Vec` will contain 4 felts, so that
    /// the total size is a valid number of words.
    pub fn to_elements(&self) -> Result<Vec<miden_processor::Felt>, String> {
        use miden_core::FieldElement;
        use miden_processor::Felt;

        let data = self.data.as_slice();
        let mut felts = Vec::with_capacity(data.len() / 4);
        let mut iter = data.iter().copied().array_chunks::<4>();
        felts.extend(iter.by_ref().map(|bytes| Felt::new(u32::from_be_bytes(bytes) as u64)));
        if let Some(remainder) = iter.into_remainder() {
            let mut chunk = [0u8; 4];
            for (i, byte) in remainder.into_iter().enumerate() {
                chunk[i] = byte;
            }
            felts.push(Felt::new(u32::from_be_bytes(chunk) as u64));
        }

        let padding = (self.size_in_words() * 4).abs_diff(felts.len());
        felts.resize(felts.len() + padding, Felt::ZERO);

        Ok(felts)
    }
}

impl Package {
    /// Create a [Package] for a [MastArtifact], using the [MasmArtifact] from which it was
    /// assembled, and the [Session] that was used to compile it.
    pub fn new(mast: MastArtifact, masm: &MasmArtifact, session: &Session) -> Self {
        let name = Symbol::intern(session.name());
        let digest = mast.digest();
        let link_libraries = session.options.link_libraries.clone();
        let mut manifest = PackageManifest {
            exports: Default::default(),
            link_libraries,
        };

        // Gater all of the rodata segments for this package
        let rodata = match masm {
            MasmArtifact::Executable(ref prog) => prog.rodatas().to_vec(),
            MasmArtifact::Library(ref lib) => lib.rodatas().to_vec(),
        };

        // Gather all of the procedure metadata for exports of this package
        if let MastArtifact::Library(ref lib) = mast {
            let MasmArtifact::Library(ref masm_lib) = masm else {
                unreachable!();
            };
            for module_info in lib.module_infos() {
                let module_path = module_info.path().path();
                let masm_module = masm_lib.get(module_path.as_ref());
                let module_span = masm_module.map(|module| module.span).unwrap_or_default();
                for (_, proc_info) in module_info.procedures() {
                    let proc_name = proc_info.name.as_str();
                    let masm_function = masm_module.and_then(|module| {
                        module.functions().find(|f| f.name.function.as_str() == proc_name)
                    });
                    let proc_span = masm_function.map(|f| f.span).unwrap_or_default();
                    let id = FunctionIdent {
                        module: Ident::new(Symbol::intern(module_path.as_ref()), module_span),
                        function: Ident::new(Symbol::intern(proc_name), proc_span),
                    };
                    let digest = proc_info.digest;
                    let signature = masm_function.map(|f| f.signature.clone());
                    manifest.exports.insert(PackageExport {
                        id,
                        digest,
                        signature,
                    });
                }
            }
        }

        Self {
            name,
            digest,
            mast,
            rodata,
            manifest,
        }
    }

    pub fn read_from_file<P>(path: P) -> std::io::Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let path = path.as_ref();
        let bytes = std::fs::read(path)?;

        Self::read_from_bytes(bytes).map_err(std::io::Error::other)
    }

    pub fn read_from_bytes<B>(bytes: B) -> Result<Self, Report>
    where
        B: AsRef<[u8]>,
    {
        use alloc::borrow::Cow;

        let bytes = bytes.as_ref();

        let bytes = bytes
            .strip_prefix(b"MASP\0")
            .ok_or_else(|| Report::msg("invalid package: missing header"))?;
        let bytes = bytes.strip_prefix(b"1.0\0").ok_or_else(|| {
            Report::msg(format!(
                "invalid package: incorrect version, expected '1.0', got '{}'",
                bytes.get(0..4).map(String::from_utf8_lossy).unwrap_or(Cow::Borrowed("")),
            ))
        })?;

        bitcode::deserialize(bytes).map_err(Report::msg)
    }

    pub fn is_program(&self) -> bool {
        matches!(self.mast, MastArtifact::Executable(_))
    }

    pub fn is_library(&self) -> bool {
        matches!(self.mast, MastArtifact::Library(_))
    }

    pub fn unwrap_program(&self) -> Arc<miden_core::Program> {
        match self.mast {
            MastArtifact::Executable(ref prog) => Arc::clone(prog),
            _ => panic!("expected package to contain a program, but got a library"),
        }
    }

    pub fn unwrap_library(&self) -> Arc<miden_assembly::Library> {
        match self.mast {
            MastArtifact::Library(ref lib) => Arc::clone(lib),
            _ => panic!("expected package to contain a library, but got an executable"),
        }
    }

    pub fn make_executable(&self, entrypoint: &FunctionIdent) -> Result<Self, Report> {
        use midenc_session::diagnostics::{SourceSpan, Span};

        let MastArtifact::Library(ref library) = self.mast else {
            return Err(Report::msg("expected library but got an executable"));
        };

        let module = library
            .module_infos()
            .find(|info| info.path().path() == entrypoint.module.as_str())
            .ok_or_else(|| {
                Report::msg(format!(
                    "invalid entrypoint: library does not contain a module named '{}'",
                    entrypoint.module.as_str()
                ))
            })?;
        let name = miden_assembly::ast::ProcedureName::new_unchecked(
            miden_assembly::ast::Ident::new_unchecked(Span::new(
                SourceSpan::UNKNOWN,
                Arc::from(entrypoint.function.as_str()),
            )),
        );
        if let Some(digest) = module.get_procedure_digest_by_name(&name) {
            let node_id = library.mast_forest().find_procedure_root(digest).ok_or_else(|| {
                Report::msg(
                    "invalid entrypoint: malformed library - procedure exported, but digest has \
                     no node in the forest",
                )
            })?;

            let exports = BTreeSet::from_iter(self.manifest.exports.iter().find_map(|export| {
                if export.digest == digest {
                    Some(export.clone())
                } else {
                    None
                }
            }));

            Ok(Self {
                name: self.name,
                digest,
                mast: MastArtifact::Executable(Arc::new(miden_core::Program::new(
                    library.mast_forest().clone(),
                    node_id,
                ))),
                rodata: self.rodata.clone(),
                manifest: PackageManifest {
                    exports,
                    link_libraries: self.manifest.link_libraries.clone(),
                },
            })
        } else {
            Err(Report::msg(format!(
                "invalid entrypoint: library does not export '{}'",
                entrypoint.display()
            )))
        }
    }
}
