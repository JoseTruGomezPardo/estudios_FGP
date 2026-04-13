<script setup lang="ts">
import { onMounted } from 'vue';
import { usePersonaViewModel } from '../../js/viewmodels/PersonaVM';

const { personas, cargando, fetchPersonas } = usePersonaViewModel();

onMounted(() => {
    fetchPersonas();
});
</script>

<template>
    <div class="container mt-4">
        <div v-if="cargando" class="alert alert-info">
            Cargando personas desde la base de datos...
        </div>

        <table class="table" v-else>
            <thead>
                <tr>
                    <th>Nombre</th>
                    <th>Apellidos</th>
                    <th>Edad</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="persona in personas" :key="persona.id">
                    <td>{{ persona.nombre }}</td>
                    <td>{{ persona.apellidos }}</td>
                    <td>{{ persona.edad }}</td>
                </tr>
                
                <tr v-if="personas.length === 0">
                    <td colspan="3" class="text-center">No hay personas registradas.</td>
                </tr>
            </tbody>
        </table>
    </div>
</template>