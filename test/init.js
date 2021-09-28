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
const getDNA = ((dnas) => (name) => (dnas[name]))({
  'agent-registration-open': path.resolve(__dirname, '../dnas/agent-registration-open/agent_registration_open_public_example.dna'),
  'agent-registration-invite-only': path.resolve(__dirname, '../dnas/agent-registration-invite-only/agent_registration_invite_only_private_example.dna'),
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
const buildPlayer = async (scenario, config, agentDNAs, autoSpawn = true) => {
  const [player] = await scenario.players([config], autoSpawn)
  const [[firstHapp]] = await player.installAgentsHapps([[agentDNAs.map(getDNA)]])

  // :SHONK: workaround nondeterministic return order for app cells, luckily nicknames are prefixed with numeric ID
  // but :WARNING: this may also break if >10 DNAs running in the same player!
  firstHapp.cells.sort((a, b) => {
    if (a.cellNick === b.cellNick) return 0
    return a.cellNick > b.cellNick ? 1 : -1
  })

  shimConsistency(scenario)

  return {
    ...firstHapp,
    player,
  }
}

// temporary method for RSM until conductor can interpret consistency
function shimConsistency(s) {
  s.consistency = () => new Promise((resolve, reject) => {
    setTimeout(resolve, 100)
  })
}

module.exports = {
  buildConfig,
  buildPlayer,
  buildRunner,
}
