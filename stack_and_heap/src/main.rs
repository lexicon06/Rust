/*
MEMORIA: STACK Y HEAP

El Stack y el Heap son abstracciones de memoria que nos permiten entender 
dónde y cómo se almacenan los datos en nuestros programas.

STACK (Pila):
- Almacena variables con tamaño conocido en tiempo de compilación.
- Es muy rápido en acceso (lectura/escritura).
- Sigue una estructura LIFO (Last In, First Out).
- La memoria se gestiona automáticamente (se libera al salir del ámbito).
- Tiene tamaño limitado.

HEAP (Montículo):
- Almacena datos con tamaño variable o desconocido en tiempo de compilación.
- Es más lento que el Stack en acceso (requiere asignación y liberación).
- Permite asignar bloques de memoria de tamaño variable.
- Requiere gestión de memoria (en Rust se hace automáticamente con el ownership).
- Tiene mayor capacidad que el Stack.

Diferencias clave:
1. Velocidad: Stack > Heap
2. Organización: Stack es ordenado (LIFO), Heap es más flexible pero menos organizado
3. Tamaño: Stack limitado, Heap más grande
4. Gestión: Stack automática, Heap requiere asignación/liberación
*/

// DEMO 1: STACK básico
fn main() {
    let x: i32 = 10;  // Se almacena en el Stack (tamaño conocido)
    let y = 20;       // Otro valor en el Stack
} // x e y son liberados aquí automáticamente

// DEMO 2: Stack con llamadas a funciones
fn foo() {
    let b = 10; // Se agrega al Stack (#2)
    let c = 20; // Se agrega al Stack (#3)
} // b y c son liberados aquí (LIFO)

fn main() {
    let a = 5;  // Se agrega al Stack (#1)
    foo();      // Durante esta ejecución se añaden b y c
} // a es liberado aquí

// DEMO 3: Heap (datos de tamaño variable)
fn main() {
    // String almacena texto de longitud variable -> Heap
    let mut s = String::from("Hola");
    s.push_str(", mundo!"); // El String puede crecer
    
    // Vector también usa Heap
    let mut v = vec![1, 2, 3];
    v.push(4); // Puede crecer dinámicamente
    
    // Box permite almacenar datos en el Heap
    let boxed = Box::new(5); // El 5 está en el Heap
} // Rust libera automáticamente estos recursos

/*
Cuando usar cada uno:
- Usa Stack para datos pequeños y de tamaño fijo (números, bools, etc.)
- Usa Heap para datos grandes o de tamaño variable (Strings, colecciones)
- En Rust, el ownership system gestiona automáticamente esta memoria
*/