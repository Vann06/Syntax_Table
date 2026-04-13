# Syntax_Table (FIRST, FOLLOW y Tabla LL(1))

Este proyecto lee una gramática desde un archivo `.txt`, calcula sus conjuntos **FIRST** y **FOLLOW**, construye la **tabla predictiva LL(1)** y reporta si la gramática es LL(1) (incluyendo conflictos por celda).

## Video

[https://youtu.be/3urJU1DQJcg](https://youtu.be/3urJU1DQJcg)

[![Video explicativo (YouTube)](https://img.youtube.com/vi/3urJU1DQJcg/hqdefault.jpg)](https://youtu.be/3urJU1DQJcg)

## Formato del archivo de gramática

- Una producción por línea.
- Formato: `A -> α | β | ...`
- Símbolos separados por espacios.
- Epsilon se escribe como `ε` (o `epsilon`). También se acepta una alternativa vacía: `A -> | b`.
- Se ignoran líneas vacías y comentarios (líneas que empiezan con `#`, `//` o `;`).

Ejemplo:

```
E -> T E'
E' -> + T E' | ε
```

## Cómo ejecutar

En la raíz del proyecto:

```
cargo run
```

Por defecto ejecuta **todos** los archivos `.txt` dentro de `./ejemplos/` (en orden alfabético).

Para ejecutar uno (o varios) archivos específicos:

```
cargo run -- ./ejemplos/ej4_parentesis.txt
cargo run -- ./ejemplos/ej1.txt ./ejemplos/ej5_conflicto_prefijo.txt
```

Tip: si pasas un directorio, ejecuta todos los `.txt` dentro de ese directorio:

```
cargo run -- ./ejemplos
```

## Gramáticas probadas

En la carpeta `ejemplos/` hay varios casos para validar que el cálculo de FIRST/FOLLOW y la construcción de la tabla sean generales.

### ej1.txt / ej2.txt — Expresiones aritméticas

- Por qué: caso clásico con precedencia (`+` y `*`) y epsilon en colas (`E'`, `T'`). Sirve para verificar que:
	- FIRST maneje alternativas con `ε`.
	- FOLLOW propague correctamente a través de producciones.
	- La tabla LL(1) tenga entradas únicas.

### ej3_listas.txt — Listas separadas por comas (LL(1))

```
S -> L
L -> id L'
L' -> , id L' | ε
```
Esta es util porque tiene una estructura comun, con recursión a la derecha y una alternativa `ε` que depende del lookahead (`,` o `$`). Es un buen caso para validar que la herramienta maneje correctamente el seguimiento de símbolos no terminales y la propagación de FIRST/FOLLOW en la tabla LL(1).
### ej4_parentesis.txt — Paréntesis balanceados (LL(1))

```
S -> ( S ) S | ε
```

recursión con alternativa `ε` donde la decisión depende del lookahead (`(` o `)`/`$`). Es un buen stress test para FOLLOW.

### ej5_conflicto_prefijo.txt — Prefijo común (NO LL(1))

```
S -> a A | a B
A -> c
B -> d
```

 ambas alternativas de `S` empiezan con el mismo terminal `a`, generando conflicto en la celda `[S, a]`. Sirve para validar detección de conflictos.

### ej6_recursion_izquierda.txt — Recursión izquierda directa (NO LL(1))

```
E -> E + T | T
T -> id
```

la recursión izquierda no es compatible con LL(1) tal cual; la tabla produce conflicto (por ejemplo en `[E, id]`). Es un caso típico que la herramienta debe reportar como NO LL(1).

### ej7_minilenguaje.txt — Mini-lenguaje (LL(1))

- Incluye: lista de sentencias, bloques `{ ... }`, `if (...) ... else ... fi`, `while`, `print`, asignación y expresiones con precedencia (`+/-/*//`) y paréntesis.
- Por qué: estresa FIRST/FOLLOW con varios no-terminales y muchas terminales distintas, y demuestra que la herramienta escala a gramáticas más “reales”.

### ej8_json.txt — JSON simplificado (LL(1))

- Incluye: objetos `{}` con pares `string : Value`, arreglos `[]`, listas separadas por comas, y literales (`true/false/null/number/string`).
- Por qué: prueba recursión y estructuras anidadas con muchas alternativas en `Value`.

### ej9_epsilon_cascada.txt — Cascada de epsilons (LL(1))

- Por qué: caso pequeño para validar propagación de FOLLOW cuando hay varios símbolos que pueden derivar `ε`.
