<template>
  <mwc-snackbar ref="create-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Create Document</span>
  
    <div style="margin-bottom: 16px">
      <mwc-textarea outlined label="Title" :value="title" @input="title = $event.target.value" required></mwc-textarea>
    </div>

    <div style="margin-bottom: 16px">
      <mwc-textarea outlined label="Content" :value="content" @input="content = $event.target.value" required></mwc-textarea>
    </div>

  
    <mwc-button 
      raised
      label="Create Document"
      :disabled="!isDocumentValid"
      @click="createDocument"
    ></mwc-button>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { Document } from './types';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
export default defineComponent({
  data(): {
    title: string;
    content: string;
  } {
    return { 
      title: '',
      content: '',
    }
  },
  computed: {
    isDocumentValid() {
    return true && this.title !== '' && this.content !== '';
    },
  },
  mounted() {
  },
  methods: {
    async createDocument() {
      const document: Document = { 
        title: this.title!,

        content: this.content!,
      };

      try {
        const record: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'kennis',
          zome_name: 'document',
          fn_name: 'create_document',
          payload: document,
        });
        this.$emit('document-created', record.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['create-error'] as Snackbar;
        errorSnackbar.labelText = `Error creating the document: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['document-created'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
