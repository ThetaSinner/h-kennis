import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleDocument(cell: CallableCell, partialDocument = {}) {
    return {
        ...{
	  title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialDocument
    };
}

export async function createDocument(cell: CallableCell, document = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "document",
      fn_name: "create_document",
      payload: document || await sampleDocument(cell),
    });
}

