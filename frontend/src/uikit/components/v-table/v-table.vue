<script setup lang="ts">
import type { TableProps } from '@uikit/components/v-table/v-table.types';


defineProps<TableProps>();
</script>

<template>

  <div class="v-table-container">
    <v-title :title="tableTitle"></v-title>

    <table>


    </table>
  </div>

</template>

<style lang="scss">

</style>