use std::collections::HashMap;

struct Biblioteca {
    libros: HashMap<String, Libro>,
}

struct Libro {
    titulo: String,
    autor: String,
    anio: u32,
}

impl Biblioteca {
    fn nueva() -> Biblioteca {
        Biblioteca {
            libros: HashMap::new(),
        }
    }

    fn agregar_libro(&mut self, libro: Libro) {
        self.libros.insert(libro.titulo.clone(), libro);
    }

    fn buscar_libro(&self, titulo: &str) -> Option<&Libro> {
        self.libros.get(titulo)
    }

    fn eliminar_libro(&mut self, titulo: &str) -> bool {
        self.libros.remove(titulo).is_some()
    }

    fn listar_libros(&self) {
        for libro in self.libros.values() {
            println!("Título: {}, Autor: {}, Año: {}", libro.titulo, libro.autor, libro.anio);
        }
    }
}

impl Libro {
    fn nuevo(titulo: String, autor: String, anio: u32) -> Libro {
        Libro { titulo, autor, anio }
    }
}

fn procesar_libros(biblioteca: &mut Biblioteca, titulos: Vec<String>) {
    for titulo in titulos {
        if let Some(libro) = biblioteca.buscar_libro(&titulo) {
            println!("Libro encontrado: {}", libro.titulo);
        } else {
            println!("Libro no encontrado: {}", titulo);
        }
    }
}

fn main() {
    let mut biblioteca = Biblioteca::nueva();

    let libro1 = Libro::nuevo("El Señor de los Anillos".to_string(), "J.R.R. Tolkien".to_string(), 1954);
    let libro2 = Libro::nuevo("Harry Potter y la Piedra Filosofal".to_string(), "J.K. Rowling".to_string(), 1997);

    biblioteca.agregar_libro(libro1);
    biblioteca.agregar_libro(libro2);

    biblioteca.listar_libros();

    let titulos = vec!["El Señor de los Anillos".to_string(), "Harry Potter y la Piedra Filosofal".to_string(), "El Código Da Vinci".to_string()];
    procesar_libros(&mut biblioteca, titulos);

    biblioteca.eliminar_libro("El Señor de los Anillos");
    biblioteca.listar_libros();
}