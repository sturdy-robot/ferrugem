use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Erro" => "Err",
        "TaOk" | "TaMec" | "Tranquilo" => "Ok",
        "Texto" => "String",
        "txt" => "str",
        "Dicionario" | "Dicionário" => "HashMap",
        "MapaArvoreB" | "MapaÁrvoreB" => "BTreeMap",
        "ListaLigada" => "LinkedList",
        "PilhaBinária" | "PilhaBinaria" => "BinaryHeap",
        "ConjHash" => "HashSet",
        "ConjArvoreB" | "ConjÁrvoreB" => "BTreeSet",
        "VetorDeck" => "VecDeque",
        "Padrão" | "Padrao" => "Default",
        "Falha" => "Error",
        "Opção" | "Opcao" => "Option",
        "Algum" => "Some",
        "Nenhum" => "None",
        "Resultado" => "Result",
        "Eu" => "Self",
        "coleções" | "colecoes" => "collections",
        "printarln" => "println",
        "printar" => "print",
        "interromper" => "break",
        "assíncrono" | "assincrono" | "asinc" => "async",
        "aguardar" => "await",
        "laço" | "laco" => "loop",
        "mover" => "move",
        "cesta" => "crate",
        "Caixa" => "Box",
        "código_inacessível" | "codigo_inacessivel" => "unreachable_code",
        "como" => "as",
        "constante" => "const",
        "atributo" => "trait",
        "tipo" => "type",
        "inseguro" => "unsafe",
        "em" => "in",
        "de" => "from",
        "din" => "dyn",
        "desempacotar" => "unwrap",
        "padrão" | "padrao" => "default",
        "como_ref" => "as_ref",
        "es" => "io",
        "externo" => "extern",
        "false" => "false",
        "função" | "funcao" => "fn",
        "super" => "super",
        "inserir" => "insert",
        "código_sem_uso" | "codigo_sem_uso" => "unused_code",

        // funções iteradores
        "iterar" => "iter",
        "para_iter" => "into_iter",
        "mapear" => "map",
        "mapear_flat" => "flat_map",
        "dobrar" => "fold",
        "drenar" => "drain",
        "coletar" => "collect",
        "encontrar" => "find",
        "pegar" => "take",
        "produto" => "product",

        // ordering
        "cmp" => "cmp",
        "Ordenar" => "Ordering",
        "MaiorQue" => "Greater",
        "MenorQue" => "Less",
        "Igual" => "Equal",
        "obter" => "get",
        "permitir" => "allow",
        "fudeu" | "deu_ruim" | "eita" | "pqp" | "caralho" => "panic",
        "módulo" | "modulo" => "mod",
        "mutável" | "mutavel" => "mut",
        "novo" => "new",
        "onde" => "where",
        "para" => "for",
        "obter_ou_inserir_com" => "get_or_insert_with",
        "principal" => "main",
        "público" | "publico" => "pub",
        "nada" => None?,
        "retornar" => "return",
        "implementar" => "impl",
        "ref" => "ref",
        "corresponder" => "match",
        "se" => "if",
        "senão" | "senao" => "else",
        "eu" => "self",
        "eis" => "let",
        "estatico" | "estático" => "static",
        "estrutura" => "struct",
        "prever" => "expect",
        "enquanto" => "while",
        "usar" => "use",
        "converter_para" => "into",
        "verdadeiro" => "true",
        "enumerador" | "enum" => "enum",
        "Literal" => "Literal",
        "macro_procedural" => "proc_macro",
        "Pontuação" | "Pontuacao" | "Ponto" => "Punct",
        "delimitador" => "delimiter",
        "estender" => "extend",
        "identificador" => "ident",
        "Vetor" => "Vec",
        "para_texto" => "to_string",
        "como_txt" => "as_str",
        "Grupo" => "Group",
        "Identificador" => "Ident",
        "FluxoDeTokens" => "TokenStream",
        "ArvoreDeTokens" | "ÁrvoreDeTokens" => "TokenTree",
        "fluxo" => "stream",
        "empurrar" => "push",
        "regras_macro!" => "macro_rules!",
        "derivar" => "derive",
        "construir" => "build",
        "funcionalidade" => "feature",
        "pdr" => "std",
        "vet" => "vec",
        "ParcialIgual" => "PartialEq",
        "ParcialOrd" => "PartialOrd",
        "Clonar" => "Clone",
        "Copiar" => "Copy",
        "Espaçamento" | "Espacamento" => "Spacing",
        "Sozinho" => "Alone",
        "Junto" => "Joint",
        "copiar" => "copy",
        "clonar" => "clone",
        "clonar_de" => "clone_from",
        "formatar" => "format",
        "igual" => "eq",
        "n_igual" => "ne",
        "IgualEstrutura" => "StructuralEq",
        "ParcialIgualEstrutura" => "StructuralPartialEq",
        "Sincronizar" => "Sync",
        "Enviar" => "Send",
        "Despinar" => "Unpin",
        "Qualquer" => "Any",
        "Emprestar" => "Borrow",
        "emprestar" => "borrow",
        "De" => "From",
        "Apropriar" => "ToOwned",
        "apropriar" => "to_owned",
        "TentarDe" => "TryFrom",
        "TentarPara" => "TryInto",
        "tentar_de" => "try_from",
        "tentar_para" => "try_into",
        "EmprestarMut" => "BorrowMut",
        "Depurar" => "Debug",
        "depurar" => "debug",
        "sa" => "fs",
        "Arquivo" => "File",
        "Escrever" => "Write",
        "escrever" => "write",
        "limpar" => "flush",
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn ferrugem(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
