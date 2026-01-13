<template>
    <view><a v-if = 'fixed' class = 'fixed'>fixed</a></view>
</template>
<script setup lang = 'ts'>
    import { ref, onBeforeMount } from 'vue';
    import SQL from '../script/sql.ts'
    import { Card } from '../script/Yu-Gi-Oh-Cards.ts';
    import { YugiohCard } from 'yugioh-card';
    import { to_data } from '../script/pic.js'

    const fixed = ref(false);

    onBeforeMount(async () => {
        const response = await fetch('./static/cards.cdb');
        const result = await SQL.find(await response.blob());
        const cardLeaf = new YugiohCard({
            resourcePath: '../static/yugioh-card',
        });
        const get = await fetch('http://localhost:5174/api/get');
		const except = await get.json();
        for (const i of result.values) {
			if (except.includes(i[0])) continue;
            const card = new Card(i as Array<string>);
            cardLeaf.setData(to_data(card));
            const result = await cardLeaf.leafer.export('jpg', true);
            const formData = new FormData();
            formData.append('file', result.data, card.id + '.jpg');
            await fetch('http://localhost:5174/api/upload', {
                method: 'POST',
                body: formData
            });
        }
        fixed.value = true;
    });

</script>