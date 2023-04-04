<script>
    import init, { add, compile } from 'agl';
    import Compile from './Compile.svelte';
</script>

{#await init()}
{:then}
<div>
    <h1>Welcome to SvelteKit</h1>
    <p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
    <Compile code="write(16,5,5); write(16,5,5); write(16,5,5);" />
</div>
{/await}
