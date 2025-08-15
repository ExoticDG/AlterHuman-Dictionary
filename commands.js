import 'dotenv/config';
import { InstallGlobalCommands } from './utils.js';

// Simple test command
const TEST_COMMAND = {
  name: 'test',
  description: 'Basic command',
  type: 1,
  integration_types: [0, 1],
  contexts: [0, 1, 2],
};

// Command containing options
const DICTIONARY_COMMAND = {
  name: 'dictionary',
  description: 'Look up a term in the Alterhuman Dictionary',
  options: [
    {
      type: 3,
      name: 'term',
      description: 'The term to look up',
      required: true,
    },
  ],
  type: 1,
  integration_types: [0, 1],
  contexts: [0, 2],
};

const ALL_COMMANDS = [TEST_COMMAND, DICTIONARY_COMMAND];

InstallGlobalCommands(process.env.APP_ID, ALL_COMMANDS);
