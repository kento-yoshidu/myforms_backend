<template>
	<div class="text-gray-600 font-black">
		<h2 class="text-4xl mb-4 md:text-7xl">{{title}}</h2>	
		<p class="text-lg">{{message}}</p>
	</div>
</template>

<script lang="ts">

export default {
	name: 'Title',
	props: {
		title: String,
		message: String
	}
}
</script>

<style>
	h2 {
		font-weight: 900;
	}
</style>