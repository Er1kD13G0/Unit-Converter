
# Unit Converter

Um conversor de unidades de medida, distância e temperatura feito em Rust 🦀


## Rodar Localmente

Clone o projeto

```bash
git clone https://github.com/Er1kD13G0/Unit-Converter
```

Vá para o diretório do projeto
```bash
cd Unit-Converter
```

Execute no terminal com os seguintes parâmetros :

| Argumento      | Descrição                      | Examplo        |
| --------------- | ------------------------------- | -------------- |
| `--value`, `-v` | Valor númerico a ser convertido | `--value 100`  |
| `--from`, `-f`  | Unidade de origem               | `--from metro` |
| `--to`, `-t`    | Unidade de destino              | `--to pes`     |


```bash
//Distancia
cargo run -- --value 1000 --from metro --to quilometro
cargo run -- --value 2.5 --from milha --to quilometro

//Peso
cargo run -- --value 500 --from grama --to kilograma
cargo run -- --value 2 --from libra --to kilograma

//Temperatura 
cargo run -- --value 100 --from celsius --to fahrenheit
cargo run -- --value 273.15 --from kelvin --to celsius
```

Unidades Suportadas
----
Distância:

    milimetro, centimetro, metro, quilometro

    polegada, pes, jarda, milha

Peso:

    miligrama, grama, kilograma

    onca, libra

Temperatura:

    celsius, fahrenheit, kelvin

## 📦 Dependência

    clap – parsing dos argumentos da linha de comando
