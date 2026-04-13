<script setup lang="ts">
import { ref } from 'vue';
import { usePersonaViewModel } from '../../js/viewmodels/PersonaVM'; // Ajusta la ruta si es necesario

// Extraemos lo que necesitamos del ViewModel
const { addPersona, cargando, error } = usePersonaViewModel();

// Creamos un objeto reactivo para capturar lo que el usuario escribe
const formulario = ref({
    nombre: '',
    apellidos: '',
    edad: null as number | null
});

// Esta función se ejecuta al pulsar el botón
const manejarEnvio = async () => {
    if (!formulario.value.nombre || !formulario.value.apellidos || !formulario.value.edad) {
        alert('Por favor, rellena todos los campos.');
        return;
    }

    // Enviamos los datos al ViewModel
    await addPersona({
        nombre: formulario.value.nombre,
        apellidos: formulario.value.apellidos,
        edad: Number(formulario.value.edad)
    });

    // Si no hubo errores, limpiamos las cajas de texto
    if (!error.value) {
        formulario.value.nombre = '';
        formulario.value.apellidos = '';
        formulario.value.edad = null;
    }
};
</script>

<template>
    <form @submit.prevent="manejarEnvio">
        <div>
            <input 
                class="input" 
                type="text" 
                v-model="formulario.nombre" 
                placeholder="nombre de la persona..."
            >
        </div>
        <div>
             <input 
                class="input"  
                type="text" 
                v-model="formulario.apellidos" 
                placeholder="apellido de la persona..."
            >
        </div>
        <div>
            <input 
                class="input" 
                type="number" 
                v-model="formulario.edad" 
                placeholder="edad de la persona..."
            >
        </div>
        <div>
            <button class="btn" type="submit" :disabled="cargando">
                {{ cargando ? 'Enviando...' : 'Enviar' }}
            </button>
        </div>
          
        <p v-if="error" style="color: red; margin-top: 10px;">{{ error }}</p>
    </form>
</template>