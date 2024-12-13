<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
// let greeting = ref('');
const rates = ref([]);
const iloscWaluty = ref(0);

const getDataFromNBP = async () => {
    const res = await fetch("https://api.nbp.pl/api/exchangerates/tables/A/?format=json");
    const jsonData = await res.json();
    console.log(jsonData);
    rates.value = jsonData[0].rates;
    console.log(rates);
}
getDataFromNBP();

const kupWalute = async (index) => {
    console.log(rates.value[index]);
}

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
        <!-- <p>{{ rates }}</p> -->

        <table>
            <tr>
                <th>Nazwa waluty</th>
                <th>Kod waluty</th>
                <th>Cena</th>
                <th>Ilość waluty do zakupu</th>
                <th></th>
            </tr>
            <tr v-for="(rate, index) in rates">
                <td>{{ rate.currency }}</td>
                <td>{{ rate.code }}</td>
                <td>{{ rate.mid }}</td>
                <td>
                    <input type="number" v-model="iloscWaluty">
                </td>
                <td>
                    <button @click="kupWalute(index)">Kup</button>
                </td>
            </tr>
        </table>

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
