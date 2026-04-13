# Syntax_Table (FIRST, FOLLOW y Tabla LL(1))

Este proyecto lee una gramática desde un archivo `.txt`, calcula sus conjuntos **FIRST** y **FOLLOW**, construye la **tabla predictiva LL(1)** y reporta si la gramática es LL(1) (incluyendo conflictos por celda).

## Formato del archivo de gramática

- Una producción por línea.
- Formato: `A -> α | β | ...`
- Símbolos separados por espacios.
- Epsilon se escribe como `ε`.

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

El programa pedirá en la terminal la ruta del `.txt` a leer.
- Presiona **Enter** para usar el default `./ejemplos/ej1.txt`.
- O escribe, por ejemplo: `./ejemplos/ej4_parentesis.txt`

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
