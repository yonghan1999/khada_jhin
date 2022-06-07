/// 查看游戏当前状态
/// 游戏大厅:None
/// 房间内:Lobby
/// 匹配中:Matchmaking
/// 找到对局:ReadyCheck
/// 选英雄中:ChampSelect
/// 游戏中:InProgress
/// 游戏即将结束:PreEndOfGame
/// 等待结算页面:WaitingForStats
/// 游戏结束:EndOfGame
/// 等待重新连接:Reconnect
pub enum GameStatusEnum {
    None,
    Lobby,
    Matchmaking,
    ReadyCheck,
    ChampSelect,
    InProgress,
    PreEndOfGame,
    WaitingForStats,
    EndOfGame,
    Reconnect
}






