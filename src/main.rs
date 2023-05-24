/*
Algoritmo de big packing

Entrada: Un conjunto de n objetos, cada uno con un peso y un valor, y una capacidad de peso W.
Salida: El subconjunto de objetos cuyo valor total es máximo, pero sin exceder W.

El Algoritmo de Envasado, también conocido como "Bin Packing", 
es utilizado para resolver el problema de colocar objetos de 
diferentes tamaños en contenedores (bins) con capacidad limitada,
con el objetivo de minimizar el número total de contenedores utilizados. 
Es un problema NP-duro, lo que significa que no existe un algoritmo eficiente q
que pueda resolverlo de manera exacta para cualquier caso.


    El enfoque básico del algoritmo voraz de envasado consiste en seguir estos pasos:

    1.Ordenar los objetos: Primero, se ordenan los objetos en orden descendente según su tamaño. Esto se hace para asegurarse de que se coloquen los objetos más grandes primero, lo que aumenta la probabilidad de utilizar menos contenedores.

    2.Inicializar los contenedores: Se crea un nuevo contenedor y se coloca el primer objeto en él.

    3.Colocar los objetos restantes: A continuación, se toma cada objeto en orden y se intenta colocarlo en los contenedores existentes. El objeto se coloca en el primer contenedor disponible donde pueda caber. Si no hay contenedores disponibles donde quepa el objeto, se crea un nuevo contenedor y se coloca allí.

    4.Repetir el paso anterior: Se continúa este proceso hasta que se hayan colocado todos los objetos en los contenedores.

*/

// Se crea una estructura para representar un contenedor , Total Operaciones Elementales: 4
struct Bin {
    capacity: u32, // Capacidad del contenedor , Operaciones Elementales: Declara, Asigna : 2
    items: Vec<u32>, // Objetos en el contenedor , Operaciones Elementales: Declara, Asigna : 2
}

// Se implementa la estructura
impl Bin {
    // Se crea un nuevo contenedor con una capacidad dada , Total Operaciones Elementales: 4
    // t(n) = 4 + 4n
    // O(n) = O(1)
    fn new(capacity: u32) -> Self {
        Bin { 
            capacity, // Capacidad del contenedor , Operaciones Elementales: Declara, Asigna : 2
            items: Vec::new(), // Objetos en el contenedor , Operaciones Elementales: Declara, Asigna : 2
        }
    }

    // Se verifica si un objeto puede caber en el contenedor, Total Operaciones Elementales: 4
    // t(n) = 4 + 4n
    // O(n) = O(1)
    fn can_fit(&self, object: u32) -> bool { 
        self.items.iter().sum::<u32>() + object <= self.capacity // Operaciones Elementales: Declara, Asigna, Aritmetica, Invocacion : 4
    }

    // Se agrega un objeto al contenedor, Total Operaciones Elementales: 3
    // t(n) = 3 + 3n
    // O(n) = O(1)
    fn add_item(&mut self, object: u32) {
        self.items.push(object); // Operaciones Elementales: Declara, Asigna, Invoca : 3
    }
}

// Se crea una función para el algoritmo de big packing, Total Operaciones Elementales: 29
// t(n) = 29 + 29n
// t(n) = O(n * k)
fn first_fit(objects: &[u32], bin_capacity: u32) -> Vec<Bin> {
    let mut bins: Vec<Bin> = Vec::new(); // Contenedores , Operaciones Elementales: Declara, Asigna, Invoca : 3

    for &object in objects { // Iterar sobre los objetos , Operaciones Elementales: Declara, Asigna, Itera : 3
        let mut placed = false; // Bandera para verificar si se colocó el objeto, Operaciones Elementales: Declara, Asigna : 2

        // Buscar un contenedor existente que pueda acomodar el objeto
        for bin in &mut bins {   // Operaciones Elementales: Declara, Asigna, Itera : 3
            if bin.can_fit(object) { // Si el objeto cabe en el contenedor , Operaciones Elementales:  Asigna, Invoca : 2
                bin.add_item(object); // Agregar el objeto al contenedor , Operaciones Elementales:  Asigna, Invoca : 2
                placed = true;      // Cambiar la bandera, Operaciones Elementales: Declara, Asigna : 2
                break;              // Salir del ciclo , Operaciones Elementales: Invoacion : 1
            }
        }

        // Si no se encuentra un contenedor existente, crear uno nuevo
        if !placed { // Si no se colocó el objeto , Operaciones Elementales: Asigna, Artimetica : 2
            let mut new_bin = Bin::new(bin_capacity); // Crear un nuevo contenedor , Operaciones Elementales: Declara, Asigna, Invoca : 3
            new_bin.add_item(object); // Agregar el objeto al contenedor , Operaciones Elementales: Asigna, Invoca : 2
            bins.push(new_bin); // Agregar el contenedor a la lista de contenedores , Operaciones Elementales: Asigna, Invoca : 2
        }
    }

    bins // Retornar los contenedores , Operaciones Elementales: Asigna : 1
}

fn main() {
    let objects = vec![3, 6, 2, 1, 5, 7, 2, 4, 1, 9]; // Objetos
    let bin_capacity = 10; // Capacidad del contenedor
 
    let packed_bins = first_fit(&objects, bin_capacity); // Algoritmo de big packing

    // Mostrar los contenedores
    for (i, bin) in packed_bins.iter().enumerate() {
        println!("Contenedor {}: {:?}", i + 1, bin.items); // Mostrar los objetos en el contenedor
    }
}
