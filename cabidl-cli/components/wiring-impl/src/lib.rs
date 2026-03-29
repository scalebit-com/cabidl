use cabidl_ai_provider::AiProvider;
use cabidl_diagram::Diagram;
use cabidl_init::Init;
use cabidl_parser::CabidlParser;
use cabidl_wiring::Wiring;

pub struct WiringImpl {
    parser: cabidl_parser_impl::CabidlParserImpl,
    diagram: cabidl_diagram_impl::DiagramImpl,
    ai_provider: cabidl_claude_code::ClaudeCodeProvider,
    init: cabidl_init_impl::InitImpl,
}

impl WiringImpl {
    pub fn new() -> Self {
        let fs = || Box::new(cabidl_filesystem_impl::RealFilesystem) as Box<dyn cabidl_filesystem::Filesystem>;

        Self {
            parser: cabidl_parser_impl::CabidlParserImpl::new(fs()),
            diagram: cabidl_diagram_impl::DiagramImpl::new(
                vec![
                    Box::new(cabidl_graphviz::GraphvizProvider),
                    Box::new(cabidl_mermaid::MermaidProvider),
                ],
            ),
            ai_provider: cabidl_claude_code::ClaudeCodeProvider::new(fs()),
            init: cabidl_init_impl::InitImpl::new(fs()),
        }
    }
}

impl Wiring for WiringImpl {
    fn parser(&self) -> &dyn CabidlParser {
        &self.parser
    }

    fn diagram(&self) -> &dyn Diagram {
        &self.diagram
    }

    fn ai_provider(&self) -> &dyn AiProvider {
        &self.ai_provider
    }

    fn init(&self) -> &dyn Init {
        &self.init
    }
}
