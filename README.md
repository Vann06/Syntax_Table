# closure_lr0 (Cerradura LR(0))

Este proyecto calcula la **cerradura (closure)** de un conjunto de **ítems LR(0)** para una gramática libre de contexto.

## Video

https://youtu.be/StYlSH2zuVM 

[![Video (YouTube)](https://youtu.be/StYlSH2zuVMD/hqdefault.jpg)](https://youtu.be/StYlSH2zuVM)


El programa:
- Lee una gramática desde archivos `.txt` dentro de la carpeta `grammars/`.
- Construye la **gramática aumentada** (agrega una nueva producción inicial `S' -> S`).
- Permite calcular la cerradura del **ítem inicial automático** o de un **ítem ingresado manualmente**.

Nota: aquí se implementa la operación de cerradura LR(0). No construye aun el autómata LR(0)/SLR ni tablas completas.

## Requisitos

- Rust y `cargo`.

## Cómo ejecutar

En la raíz del proyecto:

```
cargo run
```

El programa es interactivo: lista los `.txt` en `grammars/`, te pide elegir uno, y luego te pregunta si quieres usar el ítem inicial automático o ingresar uno manualmente.

## Formato del archivo de gramática

- Una producción por línea.
- Formato: `A -> α | β | ...`
- Símbolos separados por espacios.
- `ε` (o `epsilon`) representa epsilon.
  - También se acepta una alternativa vacía: `A -> | b`.
- Se ignoran líneas vacías y comentarios en línea completa que empiecen con `#`, `//` o `;`.
  - También se acepta comentario al final si aparece como ` # ...`.

Ejemplos válidos:

```
S -> S S + | S S * | a
```

```
S -> ( S ) | ε
```

## Formato del ítem (entrada manual)

Cuando eliges “Ingresar un ítem manualmente”, usa el formato:

```
A -> . B c
```

Reglas:
- Debe haber **exactamente un** punto `.`.
- El punto debe estar separado por espacios (por ejemplo `A -> . B` y no `A -> .B`).
- Si el punto está al final, el ítem queda completo (ejemplo: `A -> B c .`).

## Qué imprime

- La gramática original y la gramática aumentada.
- El ítem de entrada.
- Los ítems agregados durante la cerradura.
- El conjunto final de la cerradura (ordenado para lectura).

## Agregar tus propias gramáticas

1. Crea un archivo `.txt` dentro de `grammars/`.
2. Escribe tus producciones con el formato indicado.
3. Ejecuta `cargo run` y selecciona el archivo.
