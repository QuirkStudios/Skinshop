<script>
    import '../app.css';
</script>

<slot/>

<style lang="postcss">
    :global(html) {
        background-color: theme('colors.red.500');
    }
</style>