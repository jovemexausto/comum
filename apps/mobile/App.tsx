import { useCallback, useMemo, useState } from 'react'
import {
  FlatList,
  StyleSheet,
  Text,
  TextInput,
  TouchableOpacity,
  View,
} from 'react-native'
import { ComumClient, type Offer } from 'comum-js'

// ---------------------------------------------------------------------------
// Seed fixa para demo — em producao virá de secure storage
// ---------------------------------------------------------------------------
const DEMO_SEED = new Uint8Array(32).fill(0x11)
const DEMO_CAPSULE_ID = new Uint8Array(32).fill(0x46)

// ---------------------------------------------------------------------------
// App
// ---------------------------------------------------------------------------
export default function App() {
  const client = useMemo(
    () => new ComumClient(DEMO_SEED, DEMO_CAPSULE_ID),
    []
  )

  const [screen, setScreen] = useState<'home' | 'create'>('home')
  const [offers, setOffers] = useState<Offer[]>([])
  const [testimonyCount, setTestimonyCount] = useState(0)

  // Refresh lista apos qualquer mutacao
  const refresh = useCallback(() => {
    setOffers(client.knownOffers())
    setTestimonyCount(client.testimonyCount())
  }, [client])

  return (
    <View style={styles.root}>
      <Header did={client.did()} shortDid={client.shortDid()} />

      {screen === 'home' ? (
        <HomeScreen
          offers={offers}
          testimonyCount={testimonyCount}
          onCreateOffer={() => setScreen('create')}
          onAccept={(offer) => {
            client.acceptOffer(offer.id)
            client.issueReceipt(offer.id)
            refresh()
          }}
        />
      ) : (
        <CreateOfferScreen
          onSubmit={(params) => {
            client.createOffer(params)
            refresh()
            setScreen('home')
          }}
          onCancel={() => setScreen('home')}
        />
      )}

      <StatusBar count={testimonyCount} />
    </View>
  )
}

// ---------------------------------------------------------------------------
// Header
// ---------------------------------------------------------------------------
function Header({ did, shortDid }: { did: string; shortDid: string }) {
  const [copied, setCopied] = useState(false)

  return (
    <View style={styles.header}>
      <Text style={styles.headerTitle}>Comum</Text>
      <TouchableOpacity
        onPress={() => {
          // No Expo: Clipboard.setStringAsync(did)
          setCopied(true)
          setTimeout(() => setCopied(false), 1500)
        }}
      >
        <View style={styles.didBadge}>
          <Text style={styles.didLabel}>nó </Text>
          <Text style={styles.didValue}>…{shortDid}</Text>
          {copied && <Text style={styles.copiedHint}> copiado</Text>}
        </View>
      </TouchableOpacity>
    </View>
  )
}

// ---------------------------------------------------------------------------
// HomeScreen
// ---------------------------------------------------------------------------
type HomeProps = {
  offers: Offer[]
  testimonyCount: number
  onCreateOffer: () => void
  onAccept: (offer: Offer) => void
}

function HomeScreen({ offers, onCreateOffer, onAccept }: HomeProps) {
  return (
    <View style={styles.flex1}>
      <View style={styles.sectionHeader}>
        <Text style={styles.sectionTitle}>Ofertas</Text>
        <TouchableOpacity style={styles.primaryBtn} onPress={onCreateOffer}>
          <Text style={styles.primaryBtnText}>+ Nova oferta</Text>
        </TouchableOpacity>
      </View>

      {offers.length === 0 ? (
        <View style={styles.emptyState}>
          <Text style={styles.emptyText}>Nenhuma oferta ainda.</Text>
          <Text style={styles.emptySubtext}>Crie a primeira ou aguarde sync.</Text>
        </View>
      ) : (
        <FlatList
          data={offers}
          keyExtractor={(o) => o.idHex}
          contentContainerStyle={styles.list}
          renderItem={({ item }) => (
            <OfferCard offer={item} onAccept={onAccept} />
          )}
        />
      )}
    </View>
  )
}

// ---------------------------------------------------------------------------
// OfferCard
// ---------------------------------------------------------------------------
function OfferCard({
  offer,
  onAccept,
}: {
  offer: Offer
  onAccept: (o: Offer) => void
}) {
  const [accepted, setAccepted] = useState(false)

  const expires = new Date(offer.expiresAt).toLocaleDateString('pt-BR')

  return (
    <View style={styles.card}>
      <View style={styles.cardRow}>
        <Text style={styles.cardItem}>{offer.item}</Text>
        <Text style={styles.cardPrice}>
          {offer.price} {offer.currency}
        </Text>
      </View>
      <Text style={styles.cardMeta}>
        vendedor: …{offer.seller.slice(-8)} · expira {expires}
      </Text>
      {!accepted ? (
        <TouchableOpacity
          style={styles.acceptBtn}
          onPress={() => {
            setAccepted(true)
            onAccept(offer)
          }}
        >
          <Text style={styles.acceptBtnText}>Aceitar</Text>
        </TouchableOpacity>
      ) : (
        <View style={styles.receiptBadge}>
          <Text style={styles.receiptText}>✓ receipt emitido</Text>
        </View>
      )}
    </View>
  )
}

// ---------------------------------------------------------------------------
// CreateOfferScreen
// ---------------------------------------------------------------------------
type CreateProps = {
  onSubmit: (p: { item: string; price: number; currency: string; expiresAt: number }) => void
  onCancel: () => void
}

function CreateOfferScreen({ onSubmit, onCancel }: CreateProps) {
  const [item, setItem] = useState('')
  const [price, setPrice] = useState('')
  const [currency, setCurrency] = useState('')

  const valid = item.trim().length > 0 && Number(price) > 0 && currency.trim().length > 0

  return (
    <View style={styles.form}>
      <Text style={styles.formTitle}>Nova oferta</Text>

      <Text style={styles.label}>Item</Text>
      <TextInput
        style={styles.input}
        placeholder="ex: café, hora de trabalho…"
        placeholderTextColor="#888"
        value={item}
        onChangeText={setItem}
      />

      <Text style={styles.label}>Valor</Text>
      <TextInput
        style={styles.input}
        placeholder="0"
        placeholderTextColor="#888"
        keyboardType="numeric"
        value={price}
        onChangeText={setPrice}
      />

      <Text style={styles.label}>Unidade local</Text>
      <TextInput
        style={styles.input}
        placeholder="ex: feira-credito"
        placeholderTextColor="#888"
        value={currency}
        onChangeText={setCurrency}
      />

      <TouchableOpacity
        style={[styles.primaryBtn, !valid && styles.disabled]}
        disabled={!valid}
        onPress={() =>
          onSubmit({
            item: item.trim(),
            price: Number(price),
            currency: currency.trim(),
            // expira em 24h
            expiresAt: Date.now() + 86_400_000,
          })
        }
      >
        <Text style={styles.primaryBtnText}>Publicar oferta</Text>
      </TouchableOpacity>

      <TouchableOpacity style={styles.cancelBtn} onPress={onCancel}>
        <Text style={styles.cancelBtnText}>Cancelar</Text>
      </TouchableOpacity>
    </View>
  )
}

// ---------------------------------------------------------------------------
// StatusBar
// ---------------------------------------------------------------------------
function StatusBar({ count }: { count: number }) {
  return (
    <View style={styles.statusBar}>
      <View style={styles.statusDot} />
      <Text style={styles.statusText}>
        nó ativo · {count} testemunho{count !== 1 ? 's' : ''}
      </Text>
    </View>
  )
}

// ---------------------------------------------------------------------------
// Estilos
// ---------------------------------------------------------------------------
const C = {
  bg: '#0f1117',
  surface: '#1a1d27',
  border: '#2a2d3a',
  primary: '#7c6fff',
  primaryDark: '#5c4fff',
  text: '#f0f0f5',
  muted: '#888',
  success: '#4caf80',
}

const styles = StyleSheet.create({
  root: { flex: 1, backgroundColor: C.bg },
  flex1: { flex: 1 },

  // Header
  header: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    paddingHorizontal: 20,
    paddingTop: 60,
    paddingBottom: 16,
    borderBottomWidth: 1,
    borderBottomColor: C.border,
  },
  headerTitle: { fontSize: 22, fontWeight: '700', color: C.text },
  didBadge: { flexDirection: 'row', alignItems: 'center' },
  didLabel: { fontSize: 12, color: C.muted },
  didValue: { fontSize: 12, color: C.primary, fontFamily: 'monospace' },
  copiedHint: { fontSize: 11, color: C.success },

  // Section
  sectionHeader: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    paddingHorizontal: 20,
    paddingVertical: 16,
  },
  sectionTitle: { fontSize: 18, fontWeight: '600', color: C.text },

  // Empty
  emptyState: { flex: 1, alignItems: 'center', justifyContent: 'center', gap: 8 },
  emptyText: { fontSize: 16, color: C.text },
  emptySubtext: { fontSize: 13, color: C.muted },

  // List
  list: { paddingHorizontal: 16, gap: 12 },

  // Card
  card: {
    backgroundColor: C.surface,
    borderRadius: 12,
    padding: 16,
    borderWidth: 1,
    borderColor: C.border,
    gap: 6,
  },
  cardRow: { flexDirection: 'row', justifyContent: 'space-between', alignItems: 'center' },
  cardItem: { fontSize: 16, fontWeight: '600', color: C.text },
  cardPrice: { fontSize: 15, color: C.primary, fontWeight: '600' },
  cardMeta: { fontSize: 12, color: C.muted },
  acceptBtn: {
    marginTop: 8,
    backgroundColor: C.primary,
    borderRadius: 8,
    paddingVertical: 10,
    alignItems: 'center',
  },
  acceptBtnText: { color: '#fff', fontWeight: '600' },
  receiptBadge: {
    marginTop: 8,
    backgroundColor: '#1e3a2e',
    borderRadius: 8,
    paddingVertical: 10,
    alignItems: 'center',
  },
  receiptText: { color: C.success, fontWeight: '600' },

  // Form
  form: {
    flex: 1,
    paddingHorizontal: 24,
    paddingTop: 32,
    gap: 12,
  },
  formTitle: { fontSize: 20, fontWeight: '700', color: C.text, marginBottom: 8 },
  label: { fontSize: 13, color: C.muted, marginBottom: -6 },
  input: {
    backgroundColor: C.surface,
    borderRadius: 10,
    borderWidth: 1,
    borderColor: C.border,
    color: C.text,
    paddingHorizontal: 14,
    paddingVertical: 12,
    fontSize: 15,
  },

  // Buttons
  primaryBtn: {
    backgroundColor: C.primary,
    borderRadius: 10,
    paddingVertical: 13,
    paddingHorizontal: 18,
    alignItems: 'center',
  },
  primaryBtnText: { color: '#fff', fontWeight: '700', fontSize: 15 },
  cancelBtn: {
    paddingVertical: 13,
    alignItems: 'center',
  },
  cancelBtnText: { color: C.muted, fontSize: 15 },
  disabled: { opacity: 0.4 },

  // Status bar
  statusBar: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 8,
    paddingHorizontal: 20,
    paddingVertical: 12,
    borderTopWidth: 1,
    borderTopColor: C.border,
  },
  statusDot: {
    width: 8,
    height: 8,
    borderRadius: 4,
    backgroundColor: C.success,
  },
  statusText: { fontSize: 12, color: C.muted },
})
