ferrugem::ferrugem! {
    usar pdr::coleções::Dicionário como Dicio;

    atributo Eae {
        função escrever(&eu, chave: Texto, valor: Texto);
        função obter(&eu, chave: Texto) -> Resultado<Opção<&Texto>, Texto>;
    }

    estático mutável DICIONARIO: Opção<Dicio<Texto, Texto>> = Nenhum;

    estrutura Concreto;

    implementar Eae para Concreto {
        função escrever(&eu, chave: Texto, valor: Texto) {
            eis dicio = inseguro {
                DICIONARIO.obter_ou_inserir_com(Padrão::padrão)
            };
            dicio.inserir(chave, valor);
        }

        função obter(&eu, chave: Texto) -> Resultado<Opção<&Texto>, Texto> {
            se eis Algum(dicio) = inseguro { DICIONARIO.como_ref() } {
                TaMec(dicio.obter(&chave))
            } senão {
                Erro("Deu ruim, rapaziada ".para_texto())
            }
        }
    }

    público(cesta) função talvez(i: u32) -> Opção<Resultado<u32, Texto>> {
        se i % 2 == 1 {
            se i == 42 {
                Algum(Erro(Texto::de("Cacete!")))
            } senão {
                Algum(TaOk(33))
            }
        } senão {
            Nenhum
        }
    }

    asinc função exemplo() {}

    asinc função exemplo2() {
        exemplo().aguardar;
    }

    função principal() {
        eis mutável x = 31;

        corresponder x {
            42 => {
                printarln!("cacete")
            }
            _ => printarln!("Deu bom!"),
        }

        para i em 0..10 {
            eis val = laço {
                interromper i;
            };

            enquanto nada x < val {
                x += 1;
            };

            x = se eis Algum(resultado) = talvez(i) {
                resultado.desempacotar()
            } senão {
                12
            };
        }

        #[permitir(código_sem_uso)]
        função secundária() {
            deu_ruim!("Deu ruim");
            caralho!("Carai, menor, já era");
            pqp!("Perdeu, cumpadi");
            fudeu!("Já era, menor");
            eita!("Mineirês");
        }
    }
}