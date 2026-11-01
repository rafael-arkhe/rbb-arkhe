package space.arkhe.mobile.mmr

class LocalMmr {
    private val entries = mutableListOf<String>()

    fun append(entryHash: String): Int {
        entries.add(entryHash)
        return entries.size - 1
    }
}
