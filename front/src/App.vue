<template>
	<div v-if="show" class="fixed">fixed</div>
</template>

<script setup>
	import { YugiohCard } from 'yugioh-card';
	import SQL from './sql.ts'
	import { onMounted, ref } from 'vue';
	const show = ref(false);
	onMounted(async () => {
		const response = await fetch('./cards.cdb');
		const result = await SQL.find(await response.blob());
		const cardLeaf = new YugiohCard({
			resourcePath: './yugioh-card',
		});
		for (const i of result.values) {
			try {
				const card = new Card(i);
				cardLeaf.setData(to_data(card));
				const result = await cardLeaf.leafer.export('jpg', true);
				const formData = new FormData();
				formData.append('file', result.data, card.id + '.jpg');
				const response = await fetch('http://localhost:5174/api/upload', {
					method: 'POST',
					body: formData
				});
				console.log(await response.json());
			} catch (e) {
				console.error(e)
			}
		}
		setTimeout(() => {
			show.value = true;
		}, 1000 * 10);
	});

	const pic = {
		package : '',
		descriptionZoom : 0.8,
		descriptionWeight : 0.3,
		descriptionAlign : false,
		firstLineCompress : false,
		copyright : '',
		laser : '',
		rare : '',
		radius : true
	};

	const LIST_ATTR = new Map([
		[0x0, ''],
		[0x1, '地'],
		[0x2, '水'],
		[0x4, '炎'],
		[0x8, '风'],
		[0x10, '光'],
		[0x20, '暗'],
		[0x40, '神']
	])

	const LIST_RACE = new Map ([
		[0x0, ''],
		[0x1, '战士'],
		[0x2, '魔法师'],
		[0x4, '天使'],
		[0x8, '恶魔'],
		[0x10, '不死'],
		[0x20, '机械'],
		[0x40, '水'],
		[0x80, '炎'],
		[0x100, '岩石'],
		[0x200, '鸟兽'],
		[0x400, '植物'],
		[0x800, '昆虫'],
		[0x1000, '雷'],
		[0x2000, '龙'],
		[0x4000, '兽'],
		[0x8000, '兽战士'],
		[0x10000, '恐龙'],
		[0x20000, '鱼'],
		[0x40000, '海龙'],
		[0x80000, '爬虫类'],
		[0x100000, '念动力'],
		[0x200000, '幻神兽'],
		[0x400000, '创造神'],
		[0x800000, '幻龙'],
		[0x1000000, '电子界'],
		[0x2000000, '幻想魔']
	])

	const types = [
		[0x1, '怪兽'],
		[0x10, '通常'],
		[0x20, '效果'],
		[0x1000, '调整'],
		[0x800, '二重'],
		[0x200000, '反转'],
		[0x400, '同盟'],
		[0x200, '灵魂'],
		[0x400000, '卡通'],
		[0x40, '融合'],
		[0x2000, '同调'],
		[0x800000, '超量'],
		[0x1000000, '灵摆'],
		[0x80, '仪式'],
		[0x2, '魔法'],
		[0x4, '陷阱'],
		[0x10000, '速攻'],
		[0x20000, '永续'],
		[0x40000, '装备'],
		[0x80000, '场地'],
		[0x100000, '反击'],
		[0x4000000, '连接'],
		[0x2000000, '特召'],
		[0x4000, '衍生']
	]

	const list = {
		type : new Map ([
			[0x1,'monster'],
			[0x2,'spell'],
			[0x4, 'trap'],
			[0x1000000, 'pendulum']
		]),
		monster_type : new Map ([
			[0x1, 'normal'],
			[0x20, 'effect'],
			[0x40, 'fusion'],
			[0x80, 'ritual'],
			[0x2000, 'synchro'],
			[0x4000, 'token'],
			[0x800000, 'xyz'],
			[0x4000000, 'link']
		]),
		p_type : new Map ([
			[0x20, 'effect-pendulum'],
			[0x40, 'fusion-pendulum'],
			[0x80, 'ritual-pendulum'],
			[0x2000,'synchro-pendulum'],
			[0x800000, 'xyz-pendulum']
		]),
		link : new Map ([
			[0x1, 6],
			[0x2, 5],
			[0x4, 4],
			[0x8, 7],
			[0x20, 3],
			[0x40, 8],
			[0x80, 1],
			[0x100, 2]
		]),
		spell_type : new Map ([
			[0x80, 'ritual'],
			[0x10000, 'quick-play'],
			[0x20000, 'continuous'],
			[0x40000, 'equip'],
			[0x80000, 'field']
		]),
		trap_type : new Map ([
			[0x20000, 'continuous'],
			[0x100000, 'counter']
		]),
		attribute : new Map ([
			[0x1, 'earth'],
			[0x2, 'water'],
			[0x4, 'fire'],
			[0x8, 'wind'],
			[0x10, 'light'],
			[0x20, 'dark'],
			[0x40, 'divine']
		]),
	};

	function to_data (card) {
		const center_pic = `./pics/${card.id}.jpg`
		const data = {
			language: 'sc',
			font: 'custom2',
			name: card.name,
			color: '',
			align: 'left',
			gradient: false,
			gradientColor1: '#999999',
			gradientColor2: '#ffffff',
			type: 'monster',
			attribute: LIST_ATTR.get(card.attribute),
			icon: '',
			image: center_pic,
			cardType: 'normal',
			pendulumType: 'normal-pendulum',
			level: card.level & 0xffff,
			rank: card.level & 0xffff,
			pendulumScale: card.pendulum,
			pendulumDescription: '',
			monsterType: card.race != '种族 N/A' ? `${LIST_RACE.get(card.race)}族` : '',
			atkBar: true,
			atk: card.atk >= 0 ? card.atk : -1,
			def: card.def >= 0 ? card.def : -1,
			arrowList: [],
			description: card.desc,
			firstLineCompress: pic.firstLineCompress,
			descriptionAlign: pic.descriptionAlign,
			descriptionZoom: pic.descriptionZoom,
			descriptionWeight: pic.descriptionWeight,
			package: pic.package,
			password: card.id,
			copyright: pic.copyright,
			laser: pic.laser,
			rare: pic.rare,
			twentieth: false,
			radius: pic.radius,
			scale: 1,
		}

		list.type.forEach((value, key) => {
			if((card.type & key) > 0)
				data.type = value;
		});

		switch(data.type){
			case'monster' :
				list.attribute.forEach((value, key) => {
					if((card.attribute & key) > 0)
						data.attribute = value;
				});
				list.monster_type.forEach((value, key) => {
					if((card.type & key) > 0)
						data.cardType = value;
					});
				if (data.cardType == 'link')
					list.link.forEach((value, key) => {
						if((card.def & key) > 0)
							data.arrowList.push(value);
					});
				break;
			case 'spell' :
				list.spell_type.forEach((value, key) => {
					if((card.type & key) > 0)
						data.icon = value;
				});
				break;
			case 'trap' :
				list.trap_type.forEach((value, key) => {
					if((card.type & key) > 0)
						data.icon = value;
				});
				break;
			case 'pendulum' :
				list.p_type.forEach((value, key) => {
					if((card.type & key) > 0) {
						data.pendulumType = value;
					}
				});
				if (card.desc.includes('【怪兽效果】') && card.desc.includes(`←${card.pendulum} 【灵摆】 ${card.pendulum}→`)) {
					data.pendulumDescription = card.desc.split(`←${card.pendulum} 【灵摆】 ${card.pendulum}→`)[1].split('【怪兽效果】')[0].replace(/\s*[\r\n]/, '');
					data.description = card.desc.split('【怪兽效果】')[1].replace(/\s*[\r\n]/, '');
				}
				break;
		}

		if (card.desc.includes('【怪兽效果】') && card.desc.includes(`←${card.pendulum} 【灵摆】 ${card.pendulum}→`)) {
			data.pendulumDescription = card.desc.split(`←${card.pendulum} 【灵摆】 ${card.pendulum}→`)[1].split('【怪兽效果】')[0].replace(/\s*[\r\n]/, '');
			data.description = card.desc.split('【怪兽效果】')[1].replace(/\s*[\r\n]/, '');
		}

		const type_list = types.filter(item => ![0x1, 0x40, 0x80, 0x2000, 0x800000, 0x2000000].includes(item[0]));
		type_list.sort((a, b) => b[0] - a[0]);
		type_list.splice(type_list.findIndex(e => e.includes(0x1000000)), 0, [0x40, '融合'], [0x80, '仪式'], [0x2000, '同调'], [0x800000, '超量'], [0x2000000, '特殊召唤']);
		for (let i = 0; i < type_list.length; i++) {
			if ((card.type & type_list[i][0]) > 0)
				data.monsterType += `${data.monsterType == '' ? '' : '/'}${type_list[i][1]}`
		}

		if (data.cardType == 'fusion' || data.cardType == 'synchro' || data.cardType == 'xyz' || data.cardType == 'link') {
			const i = data.description.split(/\s*[\r\n]/)
			data.description = `${i[0]}\r\n${data.description.replace(i[0], '').replace(/\s*[\r\n]/g, '')}`;
		} else {
			data.description = data.description.replace(/\s*[\r\n]/g, '');
		}
		return data;
	}

	class Card {
		constructor(row) {
			this.id = row[0]
			this.ot = row[1] 
			this.alias = row[2]
			this.setcode = row[3].toString(16).padStart(16, '0').match(/.{1,4}/g);
			this.type = row[4];
			this.atk = row[5];
			this.def = row[6];
			this.level = parseInt(row[7].toString(16).slice(-4)) | 0;
			this.scale = parseInt(row[7].toString(16).slice(-4, -6)) | 0;
			this.race = row[8];
			this.attribute = row[9];
			this.category = row[10];
			this.name = row[12];
			this.desc = row[13];
			this.hint = row.slice(14, 30);
		}
	}
</script>

<style scoped></style>
