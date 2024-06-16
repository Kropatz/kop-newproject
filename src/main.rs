use std::{fs, io::stdin};

enum Operation {
    // Create a new file with the given name and content
    CreateFile(String, String),
}

#[derive(Debug)]
enum Language {
    Rust,
    Dotnet,
    Java,
    NodeJS,
    Go,
}

static SUPPORTED_LANGUAGES: [Language; 5] = [ Language::Rust, Language::Dotnet, Language::Java, Language::NodeJS, Language::Go];


fn main() {
    let language: Language;
    loop {
        println!("Please choose a language from the following: {:?}", SUPPORTED_LANGUAGES);
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_lowercase().to_string();
        if let Some(lang) = user_input_to_enum(&input) {
            language = lang;
            break;
        } else {
            println!("Invalid input. Please try again.");
        }
    }
    let operations = init_project(&language);
    for operation in operations {
        match handle_operation(operation) {
            Ok(_) => (),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn user_input_to_enum(input: &str) -> Option<Language> {
    match input {
        "rust" => Some(Language::Rust),
        "dotnet" => Some(Language::Dotnet),
        "java" => Some(Language::Java),
        "nodejs" => Some(Language::NodeJS),
        "go" => Some(Language::Go),
        _ => None,
    }
}

fn handle_operation(operation: Operation) -> Result<(), String> {
    match operation {
        Operation::CreateFile(name, content) => {
            if (fs::metadata(&name)).is_ok() {
                return Err(format!("File {} already exists", name));
            }
            let result = fs::write(name.clone(), content);
            if result.is_ok() {
                Ok(())
            } else {
                Err(format!("Failed to create file {}", name))
            }
        }
    }
}

fn init_project(language: &Language) -> Vec<Operation> {
    let shell = create_nix_shell(language);
    let direnv = create_direnv(language);
    println!("Creating a new {:?} project", language);
    vec![
        Operation::CreateFile("shell.nix".to_string(), shell),
        Operation::CreateFile(".envrc".to_string(), direnv.to_string()),
    ]
}

fn create_nix_shell(language: &Language) -> String {
    let (packages, hook) = match language {
        Language::Rust => (
            vec!["cargo", "rustc", "rustfmt"],
            vec!["export LD_LIBRARY_PATH=$NIX_LD_LIBRARY_PATH"],
        ),
        Language::Go => (vec!["go"], vec![]),
        Language::Java => (vec!["jdk"], vec![]),
        Language::NodeJS => (vec!["nodejs_20"], vec![]),
        Language::Dotnet => (
            vec!["dotnet-sdk"],
            vec![
                "export DOTNET_CLI_TELEMETRY_OPTOUT=1",
                "export DOTNET_ROOT=${pkgs.dotnet-sdk}",
            ],
        ),
    };
    format!(
        r#"{{ pkgs ? import <nixpkgs> {{}} }}:
pkgs.mkShell rec {{
  buildInputs = with pkgs; [ 
    {packages}
  ];

  shellHook = ''
    {hooks}
  '';
}}"#,
        packages = packages.join("\n    "),
        hooks = hook.join("\n    ")
    )
}

fn create_direnv(language: &Language) -> &str {
    match language {
        _ => "use nix",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shell_test_rust() {
        let string = create_nix_shell(&Language::Rust);
        assert!(string.contains("cargo"));
        assert!(string.contains("rustc"));
        assert!(string.contains("rustfmt"));
        assert!(string.contains("LD_LIBRARY_PATH"));
    }
    #[test]
    fn shell_test_go() {
        let string = create_nix_shell(&Language::Go);
        assert!(string.contains("go"));
    }
    #[test]
    fn shell_test_java() {
        let string = create_nix_shell(&Language::Java);
        assert!(string.contains("jdk"));
    }
    #[test]
    fn shell_test_nodejs() {
        let string = create_nix_shell(&Language::NodeJS);
        assert!(string.contains("nodejs_20"));
    }
    #[test]
    fn shell_test_dotnet() {
        let string = create_nix_shell(&Language::Dotnet);
        assert!(string.contains("dotnet-sdk"));
        assert!(string.contains("DOTNET_CLI_TELEMETRY_OPTOUT"));
        assert!(string.contains("DOTNET_ROOT"));
    }

    #[test]
    fn direnv_test() {
        for language in &SUPPORTED_LANGUAGES  {
            let string = create_direnv(&language);
            assert_eq!(string, "use nix");
        }
    }
}
