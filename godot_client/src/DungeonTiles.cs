using Godot;
using System;

using CardGame;
public class DungeonTiles : DungeonTilesFs
{
    [Signal]
    public delegate void SetPlayerPos(Vector2 a, Image image);
    [Signal]
    public delegate void EnteredBattle();
    [Signal]
    public delegate void UpdateDungeon(int a);
}
