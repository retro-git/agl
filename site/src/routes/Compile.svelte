<script>
    import init, { compile } from 'agl';
    //prop to hold string to compile
    export let code = "";
</script>

<p>{compile(code)}</p>
