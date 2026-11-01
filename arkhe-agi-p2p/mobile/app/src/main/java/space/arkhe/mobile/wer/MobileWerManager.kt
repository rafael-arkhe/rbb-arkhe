package space.arkhe.mobile.wer

class MobileWerManager {
    fun recordExecution(appId: String, actionName: String): String {
        return "MobileWER:recorded:$appId:$actionName"
    }
}
