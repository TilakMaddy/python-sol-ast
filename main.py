import subprocess
import json

def prepare_ast():
    quiet_mode = False # Change this to True if you don't want to see cargo stuff
    command = ""
    if quiet_mode != True:
        command = "cargo run --manifest-path ast-gen/Cargo.toml"
    else:
        command = "cargo run -q --manifest-path ast-gen/Cargo.toml"
    subprocess.run(command.split(" ")) 

def main():
    print("Waiting for solidity-ast-rs to give the report!")
    prepare_ast()
    print("Done! AST written to output.json")

    f = open("output.json", "r")
    contexts = json.loads(f.read())['content']

    for ast in contexts:
        # Each ast is 1 context
        print("Solidity Version", ast['version'])
        print("File path", ast['file_path'])
        print("AST JSON", ast['ast_json'])

if __name__ == "__main__":
    main()
