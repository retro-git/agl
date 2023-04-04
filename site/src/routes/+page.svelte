<script lang="ts">
    import { AceEditor } from "svelte-ace";
    import "brace/mode/json";
    import "brace/theme/twilight";

    import init, { compile } from "agl";
    import { HSplitPane, VSplitPane } from "svelte-split-pane";
    import Compile from "./Compile.svelte";

    let code_editor_text = "";
    let compiled_editor_text = "";

    function onInput(obj) {
        code_editor_text = obj.detail;
        try {
            compiled_editor_text = compile(obj.detail);
        } catch (e) {}
    }
</script>

{#await init() then}
    <div>
        <!-- <Compile code="write(16,5,5); write(16,5,5); write(16,5,5);" /> -->
        <h2 class="title">Abstract GameShark Language</h2>
        <HSplitPane leftPaneSize="50%" rightPaneSize="50%" minLeftPaneSize="50px" minRightPaneSize="50px">
            <left slot="left">
                <AceEditor
                    on:selectionChange={(obj) => console.log(obj.detail)}
                    on:paste={(obj) => console.log(obj.detail)}
                    on:input={onInput}
                    on:focus={() => console.log("focus")}
                    on:documentChange={(obj) =>
                        console.log(`document change : ${obj.detail}`)}
                    on:cut={() => console.log("cut")}
                    on:cursorChange={() => console.log("cursor change")}
                    on:copy={() => console.log("copy")}
                    on:init={(editor) => console.log(editor.detail)}
                    on:commandKey={(obj) => console.log(obj.detail)}
                    on:changeMode={(obj) =>
                        console.log(`change mode : ${obj.detail}`)}
                    on:blur={() => console.log("blur")}
                    width="100%"
                    height="800px"
                    lang="json"
                    theme="twilight"
                    options={{fontSize: "14pt"}}
                    value={code_editor_text}
                />
            </left>
            <right slot="right">
                <AceEditor
                    on:selectionChange={(obj) => console.log(obj.detail)}
                    on:paste={(obj) => console.log(obj.detail)}
                    on:input={(obj) => console.log(obj.detail)}
                    on:focus={() => console.log("focus")}
                    on:documentChange={(obj) =>
                        console.log(`document change : ${obj.detail}`)}
                    on:cut={() => console.log("cut")}
                    on:cursorChange={() => console.log("cursor change")}
                    on:copy={() => console.log("copy")}
                    on:init={(editor) => console.log(editor.detail)}
                    on:commandKey={(obj) => console.log(obj.detail)}
                    on:changeMode={(obj) =>
                        console.log(`change mode : ${obj.detail}`)}
                    on:blur={() => console.log("blur")}
                    width="100%"
                    height="800px"
                    lang="json"
                    theme="twilight"
                    readonly={true}
                    options={{fontSize: "14pt"}}
                    value={compiled_editor_text}
                />
            <!-- <button on:click={() => compiled_editor_text = compile(obj.detail)}>Compile</button> -->
        </HSplitPane>
    </div>
{/await}

<style>
    :global(body) {
        background-color: #0e1012;
        color: #ffffff;
    }

    :global(.ace_scrollbar) {
        display: none !important;
    }

    .title {
        text-align: center;
    }
</style>
