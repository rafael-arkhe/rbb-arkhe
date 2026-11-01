package space.arkhe.mobile.attestation

class StrongBoxAttestation {
    fun getTitanM2Quote(): ByteArray {
        return "TITAN_M2_SEALED_BOOT_QUOTE".toByteArray()
    }
}
