import { ref } from 'vue';
import type { Persona } from '../models/PersonaModel';

// 🔥 LA SOLUCIÓN ESTÁ AQUÍ: Sacamos los "ref" fuera de la función.
// Al estar aquí arriba, solo se crean una vez. Todos los componentes 
// que usen este ViewModel mirarán exactamente a estas mismas variables.
const personas = ref<Persona[]>([]);
const cargando = ref<boolean>(false);
const error = ref<string | null>(null);

export function usePersonaViewModel() {

    // 1. Función GET para cargar la lista inicial
    const fetchPersonas = async () => {
        cargando.value = true;
        error.value = null;
        try {
            const response = await fetch('http://127.0.0.1:8000/getpersonas');
            const result = await response.json();
            
            personas.value = result.data; 
        } catch (err) {
            error.value = 'Error al cargar las personas';
            console.error(err);
        } finally {
            cargando.value = false;
        }
    };

    // 2. Función POST para guardar y actualizar la vista al instante
    const addPersona = async (nuevaPersona: Omit<Persona, 'id'>) => {
        cargando.value = true;
        error.value = null;
        try {
            const response = await fetch('http://127.0.0.1:8000/addpersonas', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Accept': 'application/json'
                },
                body: JSON.stringify(nuevaPersona) 
            });

            const result = await response.json();

            if (response.ok) {
                // Volvemos a pedir la lista completa a Laravel para ir sobre seguro
                // y que no haya ningún desajuste.
                await fetchPersonas();
                
                return result;
            } else {
                error.value = result.message || 'Error al guardar';
            }
        } catch (err) {
            error.value = 'Error de conexión con el servidor';
            console.error(err);
        } finally {
            cargando.value = false;
        }
    };

    return {
        personas,
        cargando,
        error,
        fetchPersonas,
        addPersona, 
    };
}