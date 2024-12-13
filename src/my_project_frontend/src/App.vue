<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
// let greeting = ref('');
const rates = ref([]);
const iloscWaluty = ref([]);

const getDataFromNBP = async () => {
    const res = await fetch("https://api.nbp.pl/api/exchangerates/tables/A/?format=json");
    const jsonData = await res.json();
    console.log(jsonData);
    rates.value = jsonData[0].rates;
    console.log(rates);
    iloscWaluty.value = rates.value.map(() => 0);
}
getDataFromNBP();

const calculateCost = async (iloscWaluty,index) => {
    const ilosc = BigInt(iloscWaluty);
    const cena = BigInt(rates.value[index].mid * 10e16);

    const koszt = await my_project_backend.calculate_currency_price(ilosc, cena);
    return koszt/BigInt(10e16);
}

const kupWalute = async () => {
    const koszta = await Promise.all(iloscWaluty.value.map((ilosc,index) => calculateCost(ilosc,index)));
    const sum = koszta.reduce(
        (al, val) => {
            al + val
        },
        0n
    )
    console.log(sum);
}

const onChange = (e,index) => {
    iloscWaluty.value[index] = e.target.value;
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
         {{iloscWaluty}}

        <table>
            <tr>
                <th>Nazwa waluty</th>
                <th>Kod waluty</th>
                <th>Cena</th>
                <th>Ilość waluty do zakupu</th>
            </tr>
            <tr v-for="(rate, index) in rates">
                <td>{{ rate.currency }}</td>
                <td>{{ rate.code }}</td>
                <td>{{ rate.mid }}</td>
                <td>
                    <input type="number" @change="(e) => onChange(e,index)" @keydown="(e) => onChange(e, index)">
                </td>
            </tr>
        </table>
        <button @click="kupWalute()">Kup</button>

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
