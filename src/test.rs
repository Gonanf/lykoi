#[cfg(test)]
mod tests {
    extern crate graphviz_rust;
    use graphviz_rust::dot_structures::*;
    use graphviz_rust::parse;
    use graphviz_rust::printer::DotPrinter;
    use graphviz_rust::printer::PrinterContext;

    #[test]
    fn parse_test() {
        let dot_graph =
            r#"
                 graph G {
                    layout=neato
                    run -- intr;
                    intr -- runbl;
                    runbl -- run;
                    run -- kernel;
                    kernel -- zombie;
                    kernel -- sleep;
                    kernel -- runmem;
                    sleep -- swap;
                    swap -- runswap;
                    runswap -- new;
                    runswap -- runmem;
                    new -- runmem;
                    sleep -- runmem;
                }
            "#;

        let graph: Result<Graph, String> = parse(dot_graph);
        graph.expect("Err").print(&mut PrinterContext::default());
    }

}