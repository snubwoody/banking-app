<script lang="ts">

</script>

<section>   
    <h6>Accounts</h6>
    <ul>
        <li>RBC Savings</li>
        <li>Absa Savings</li>
        <li>Credit card</li>
    </ul>
    <button>Add account</button>
</section>

<style>
    @reference "../styles/app.css";

    section{
        display: flex;
        flex-direction: column;
        gap: 20px;
        align-items: start;
        background: white;
        border-radius: var(--radius-md);
        padding: --spacing(2.5);
        box-shadow: var(--shadow-sm);
    }

    ul{
        width: 100%;
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(250px,1fr));
        gap: 24px;

        li{
            background: var(--color-neutral-50);
            border-radius: var(--radius-md);
            padding: 12px 16px;
        }
    }
</style>