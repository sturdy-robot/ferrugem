# ferrugem

![ferrugem](ferrugemrust.png)

Aren't you _pistola_ from writing Rust programs in English? Do you like saying
"caralho" a lot? Would you like to try something different, in an exotic and
beautiful language? Would you want to bring some HUE HUE BR BR touch to your
programs?

**ferrugem** (PT-BR for _Rust_) is here to save your day, as it allows you to
write Rust programs in Brazilian Portuguese, using BR keywords, BR function names,
and BR idioms.

You don't feel at ease using only PT-BR words? Don't worry!
Ferrugem is fully compatible with English-Rust, so you can mix both at your
convenience.

Here's couple examples of what can be achieved with Ferrugem:

## Olá, Mundo!

```rust
ferrugem::ferrugem! {
    função principal() {
        printarln!("Olá, Mundo!");
    }
}
```

## struct and impl (aka estrutura e implementar)

```rust
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
```

## Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax.

## pra que fazer isso?

* There are [French](https://github.com/bnjbvr/rouille), [Dutch](https://github.com/jeroenhd/roest), and [German](https://github.com/michidk/rost) versions of Rust, we must bring the BR spirit to the fight as well!
* Nóis é BR memo caraio
* Pq eu quis!

## Agradecimentos

- [German](https://github.com/michidk/rost): I saw this first and decided to do a BR version as well. No one has done it so far, so I took my chance.

## Licença

[WTFPL](http://www.wtfpl.net/).
