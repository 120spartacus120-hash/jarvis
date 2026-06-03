<script lang="ts">
    import { Container } from "@svelteuidev/core"
    import Header from "@/components/Header.svelte"
    import UpdateBanner from "@/components/UpdateBanner.svelte"
</script>

<Container fluid id="wrapper">
    <Header />
    <UpdateBanner />
    <main>
        <slot></slot>
    </main>
</Container>
