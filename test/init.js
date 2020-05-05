/**
 * Test environment bootstrap
 *
 * @package Agent registration hApp
 * @author  pospi <pospi@spadgos.com>
 * @since   2020-05-05
 */

require('source-map-support').install()

const path = require('path')
const tape = require('tape')

const { Orchestrator, Config, combine, tapeExecutor, localOnly } = require('@holochain/tryorama')

process.on('unhandledRejection', error => {
  console.error('unhandled rejection:', error)
  // delay exit so that debug logs have time to pipe through
  setTimeout(() => {
    process.exit(1)
  }, 500)
})

// DNA loader, to be used with `buildTestScenario` when constructing DNAs for testing
const getDNA = ((dnas) => (path) => (Config.dna(dnas[path], path)))({
  'agent-registration-open': path.resolve(__dirname, '../happs/agent-registration-open/dist/agent-registration-open.dna.json'),
  'agent-registration-invite-only': path.resolve(__dirname, '../happs/agent-registration-invite-only/dist/agent-registration-invite-only.dna.json'),
})

/**
 * Construct a test scenario out of the set of input instances & bridge configurations
 *
 * @param  {object} instances mapping of instance IDs to DNAs (@see getDNA)
 * @param  {object} bridges   (optional) mapping of bridge IDs to DNA instance ID pairs
 * @return Try-o-rama config instance for creating 'players'
 */
const buildConfig = (instances, bridges) => {
  return Config.gen(instances, {
    bridges: Object.keys(bridges || {}).reduce((b, bridgeId) => {
      b.push(Config.bridge(bridgeId, ...bridges[bridgeId]))
      return b
    }, []),
    network: {
      type: 'sim2h',
      sim2h_url: 'ws://localhost:9000',
    },
    // logger: Config.logger(!!process.env.VERBOSE_DNA_DEBUG),
  })
}

/**
 * Create a test scenario orchestrator instance
 */

const buildRunner = () => new Orchestrator({
  middleware: combine(
    tapeExecutor(tape),
    localOnly,
  ),
})

/**
 * Create per-agent interfaces to the DNA
 */

const buildPlayer = async (scenario, playerName, config) => {
  const players = await scenario.players({ [playerName]: config }, true)
  return players[playerName]
}

module.exports = {
  getDNA,
  buildConfig,
  buildPlayer,
  buildRunner,
}
