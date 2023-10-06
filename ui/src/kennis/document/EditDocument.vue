<template>
  <mwc-snackbar ref="update-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Edit Document</span>
      <div style="margin-bottom: 16px">
      <mwc-textarea outlined label="Title" :value="title" @input="title = $event.target.value" required></mwc-textarea>
      </div>

      <div style="margin-bottom: 16px">
      <mwc-textarea outlined label="Content" :value="content" @input="content = $event.target.value" required></mwc-textarea>
      </div>



    <div style="display: flex; flex-direction: row">
      <mwc-button
        outlined
        label="Cancel"
        @click="$emit('edit-canceled')"
        style="flex: 1; margin-right: 16px;"
      ></mwc-button>
      <mwc-button 
        raised
        label="Save"
        :disabled="!isDocumentValid"
        @click="updateDocument"
        style="flex: 1;"
      ></mwc-button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { Document } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { decode } from '@msgpack/msgpack';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';

export default defineComponent({
  data(): {
    title: string;
    content: string;
  } {
    const currentDocument = decode((this.currentRecord.entry as any).Present.entry) as Document;
    return { 
      title: currentDocument.title,
      content: currentDocument.content,
    }
  },
  props: {
    originalDocumentHash: {
      type: null,
      required: true,
    },
    currentRecord: {
      type: Object,
      required: true
    }
  },
  computed: {
    currentDocument() {
      return decode((this.currentRecord.entry as any).Present.entry) as Document;
    },
    isDocumentValid() {
      return true && this.title !== '' && this.content !== '';
    },
  },
  mounted() {
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord input is required for the EditDocument element`);
    }
    if (this.originalDocumentHash === undefined) {
      throw new Error(`The originalDocumentHash input is required for the EditDocument element`);
    }
  },
  methods: {
    async updateDocument() {

      const document: Document = { 
        title: this.title,
        content: this.content,
      };

      try {
        const updateRecord: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'kennis',
          zome_name: 'document',
          fn_name: 'update_document',
          payload: {
            original_document_hash: this.originalDocumentHash,
            previous_document_hash: this.currentRecord.signed_action.hashed.hash,
            updated_document: document
          }
        });
        this.$emit('document-updated', updateRecord.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['update-error'] as Snackbar;
        errorSnackbar.labelText = `Error updating the document: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['document-updated', 'edit-canceled'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
