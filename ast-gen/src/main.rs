use serde::Serialize;
use solidity_ast_rs::{
    DerivedAstEvmInfo, ProjectConfigInputBuilder, Result, derive_ast_and_evm_info,
};
use std::{fs::File, io::Write, path::Path};

pub fn ast_info(root: &str) -> Result<DerivedAstEvmInfo> {
    let config = ProjectConfigInputBuilder::new(Path::new(root)).build()?;
    derive_ast_and_evm_info(&config)
}

#[derive(Serialize, Default)]
struct ASTStuff {
    content: Vec<AST>,
}

#[derive(Serialize, Default)]
struct AST {
    file_path: String,
    ast_json: String,
    version: String,
}

fn main() {
    let mut output = ASTStuff::default();

    let asts = ast_info("defi-app").unwrap();
    for ast in asts.versioned_asts {
        for (path, ast_content) in ast.compiler_output.sources {
            output.content.push(AST {
                version: ast.version.to_string(),
                file_path: path.to_string_lossy().to_string(),
                ast_json: ast_content.ast.unwrap(),
            });
        }
    }

    let json_string = serde_json::to_string(&output).unwrap();

    let mut file = File::create("output.json").expect("openning file failed");
    file.write_all(json_string.as_bytes())
        .expect("failed to write to file");
}
