export interface Persona {
    id?: number; // El ID es opcional porque al crear una nueva persona aún no lo tiene
    nombre: string;
    apellidos: string;
    edad: number;
}