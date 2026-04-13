<script setup lang="ts">
import Form from './widgets/form/Form.vue';
import Tabla from './widgets/table/Tabla.vue';

</script>

<template>
    <main style="display: flex;justify-content: center;align-items: center;flex-direction: column;height: 100vh;">
        <div style="width: 70%;">
            <h1>Introduce o registra una persona:</h1>
        <Form/>
        <div style="margin-top: 3rem;">
            <Tabla/>
        </div>
        </div>
        
    </main>
</template>