import fs from 'fs';
import { parseString } from 'xml2js';

// Read the XML file
const xml = fs.readFileSync('dictionary.xml', 'utf8');

let dictionaryData;

// Parse the XML
parseString(xml, (err, result) => {
  if (err) {
    console.error('Error parsing XML:', err);
    return;
  }
  dictionaryData = result.dictionary.entry;
});

/**
 * Get the definition for a given term.
 * @param {string} term - The term to search for.
 * @returns {{term: string, definition: string}|null} The main term and definition, or null if not found.
 */
export function getDefinition(term) {
  if (!dictionaryData) {
    return null;
  }

  const entry = dictionaryData.find((e) => {
    // The entry can have multiple terms. We check if any of them match.
    return e.term.some((entryTerm) => {
      // The term from XML might have a colon at the end, so we'll handle that.
      // Also, we'll do a case-insensitive search.
      const cleanedEntryTerm = entryTerm.replace(/:$/, '').toLowerCase();
      return cleanedEntryTerm === term.toLowerCase();
    });
  });

  if (entry) {
    // The main term is the first one, without the colon.
    const mainTerm = entry.term[0].replace(/:$/, '');
    return {
      term: mainTerm,
      definition: entry.definition[0],
    };
  }

  return null;
}
