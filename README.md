# hola-cocacola — Fundamentos de Rust

Repositorio de práctica: cada tema es un programa independiente en `examples/`.

## Cómo ejecutar un tema

```bash
cargo run --example 04_condicionales
```

Para compilar todos los temas de una vez:

```bash
cargo build --examples
```

## Temas vistos

| #  | Tema                                   | Archivo |
|----|----------------------------------------|---------|
| 01 | Variables, shadowing y alcance (scope) | [01_variables_shadowing.rs](examples/01_variables_shadowing.rs) |
| 02 | Referencias (`&`)                      | [02_referencias.rs](examples/02_referencias.rs) |
| 03 | Operadores y expresiones               | [03_operadores_expresiones.rs](examples/03_operadores_expresiones.rs) |
| 04 | Condicionales (`if` como expresión)    | [04_condicionales.rs](examples/04_condicionales.rs) |
| 05 | Préstamo mutable (`&mut`)              | [05_prestamos_mut.rs](examples/05_prestamos_mut.rs) |
| 06 | Préstamos con structs y `Vec`          | [06_prestamos_structs_vec.rs](examples/06_prestamos_structs_vec.rs) |
| 07 | `Option<T>` y `match`                  | [07_option_basico.rs](examples/07_option_basico.rs) |
| 08 | `Option<T>` con structs                | [08_option_con_struct.rs](examples/08_option_con_struct.rs) |
| 09 | `match`                                | [09_expresion_match.rs](examples/09_expresion_match.rs) |
| 10 | `match`                                | [10_match_nulo.rs](examples/10_match_nulo.rs) |
| 11 | `match`                                | [11_match_tupla.rs](examples/11_match_tupla.rs) |
| 12 | `match`                                | [12_match_incrementar.rs](examples/12_match_incrementar.rs) |
| 13 | `loop`                                 | [13_bucles.rs](examples/13_bucles.rs) |
| 14 | `while`                                | [14_while.rs](examples/14_while.rs) |
| 15 | `while`, `let`                         | [15_while_let.rs](examples/15_while_let.rs) |
| 16 | `for`                                  | [16_while_let.rs](examples/16_for_.rs) |
## Agregar un tema nuevo

1. Crea `examples/NN_nombre_del_tema.rs` (siguiente número, en snake_case).
2. Escribe un `fn main() { ... }` dentro.
3. Pruébalo con `cargo run --example NN_nombre_del_tema`.
4. Agrega una fila a la tabla de arriba.
