<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
// let greeting = ref('');

const getDataFromNBP = async () => {
    const res = await fetch("https://api.nbp.pl/api/exchangerates/tables/A/?format=json");
    const jsonData = await res.json();
    console.log(jsonData);
    const rates = jsonData[0].rates;
    console.log(rates);
}
getDataFromNBP();

// async function handleSubmit(e) {
//   e.preventDefault();
//   const target = e.target;
//   const name = target.querySelector('#name').value;
//   await my_project_backend.greet(name).then((response) => {
//     greeting.value = response;
//   });
// }
</script>

<template>
    <main>

        <!-- <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <label for="name">Enter your name: &nbsp;</label>
      <input id="name" alt="Name" type="text" />
      <button type="submit">Click Me!</button>
    </form>
    <section id="greeting">{{ greeting }}</section> -->
    </main>
</template>
