<template>
    <header>
        <h4 class="repository__title">
            <img class="project__logo" src="https://avatars0.githubusercontent.com/u/69631?v=3&s=200" alt="Logo of Facebook">
            <a href="https://github.com/facebook" target="_blank">facebook</a>
            /
            <a href="https://github.com/facebook/flow" target="_blank">flow</a>
        </h4>

        <ul class="repository__properties">
            <li class="repository__property">
                <span class="language__dot" style="background-color: #3be133;"></span>
                OCaml
            </li>

            <li class="repository__property">
                <svg class="icon icon--star" viewBox="0 0 14 16">
                    <path d="M14 6l-4.9-.64L7 1 4.9 5.36 0 6l3.6 3.26L2.67 14 7 11.67 11.33 14l-.93-4.74z"></path>
                </svg>
                10,463
            </li>

            <li class="repository__property">Updated 59 min ago</li>
        </ul>

        Adds static typing to JavaScript to improve developer productivity and code quality.
    </header>
</template>

<script>
    export default {
        name: 'repository'
    }
</script>

<style>
    .project__logo{
        width: var(--logo-size);
        max-height: var(--logo-size);
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

    .icon{
        height: 18px;
        fill: #444;
        vertical-align: middle;
    }

    .icon.icon--star{
        position: relative;
        fill: #ffb700;
        top: -2px;
    }

    .icon.icon--release{
        height: 24px;
        margin-right: 4px;
    }
</style>
