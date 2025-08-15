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
 * @returns {string|null} The definition of the term, or null if not found.
 */
export function getDefinition(term) {
  if (!dictionaryData) {
    return 'The dictionary data is not available.';
  }

  const entry = dictionaryData.find((e) => {
    // The term from XML might have a colon at the end, so we'll handle that.
    // Also, we'll do a case-insensitive search.
    const entryTerm = e.term[0].replace(/:$/, '').toLowerCase();
    return entryTerm === term.toLowerCase();
  });

  if (entry) {
    return entry.definition[0];
  }

  return null;
}
