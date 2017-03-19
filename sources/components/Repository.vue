<template>
    <header>
        <h4 class="repository__title" v-if="value.owner">
            <img class="project__logo" :src="value.owner.avatarURL" :alt="value.owner.login ? `Logo of ${value.owner.login.charAt(0).toUpperCase() + value.owner.login.slice(1)}` : 'Logo'">
            <a :href="`https://github.com${value.owner.path}`" target="_blank">{{ value.owner.login }}</a>
            /
            <a :href="`https://github.com${value.path}`" target="_blank">{{ value.name }}</a>
        </h4>

        <ul class="repository__properties">
            <li class="repository__property" v-if="value.language">
                <span class="language__dot" :style="{ backgroundColor: value.language.color }"></span>
                {{ value.language.name }}
            </li>

            <li class="repository__property">
                <svg class="icon icon--star" viewBox="0 0 14 16">
                    <path d="M14 6l-4.9-.64L7 1 4.9 5.36 0 6l3.6 3.26L2.67 14 7 11.67 11.33 14l-.93-4.74z"></path>
                </svg>
                {{ value.stargazers | number }}
            </li>

            <li class="repository__property" v-if="value.release">
				Published
				{{ value.release.publishedAt | relativeDate }}
			</li>
        </ul>

        <div v-html="value.description"></div>
    </header>
</template>

<script>
    export default {
        name: 'repository',
        props: {
            value: {
                type: Object,
                required: true
            }
        }
    }
</script>

<style>
    .project__logo{
        width: var(--logo-size); height: var(--logo-size);
        margin-right: 10px;
        vertical-align: middle;
        border-radius: 50%;
        overflow: hidden;
    }

    .repository__title{
        margin-top: 0;
        margin-bottom: 0;
        font-size: 2rem;
        line-height: var(--logo-size);
        color: var(--light-blue);
    }

    .repository__properties{
        list-style: none;
        padding-left: 0;
    }

    .repository__property{
        display: inline-block;
        margin-right: 15px;
    }

    .language__dot{
        display: inline-block;
        width: 14px; height: 14px;
        margin-right: 2px;
        border-radius: 50%;
    }

    .icon.icon--star{
        position: relative;
        fill: #ffb700;
        top: -2px;
    }
</style>
