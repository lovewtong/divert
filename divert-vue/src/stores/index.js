// src/stores/counter.js

export const state = () => ({
    playlists: [], // 存储播放列表数据
    selectedPlaylists: [] // 存储选中的播放列表
  });
  
  export const getters = {
    allPlaylists: state => state.playlists,
    selectedPlaylists: state => state.selectedPlaylists
  };
  
  export const mutations = {
    SET_PLAYLISTS(state, playlists) {
      state.playlists = playlists;
    },
    SET_SELECTED_PLAYLISTS(state, selected) {
      state.selectedPlaylists = selected;
    },
    TOGGLE_PLAYLIST_SELECTION(state, playlistId) {
      const index = state.selectedPlaylists.indexOf(playlistId);
      if (index !== -1) {
        state.selectedPlaylists.splice(index, 1); // 如果已选中，取消选择
      } else {
        state.selectedPlaylists.push(playlistId); // 如果未选中，添加到选择列表中
      }
    }
  };
  
  export const actions = {
    fetchPlaylists({ commit }) {
      // 假设这里有一个 API 调用来获取播放列表数据
      const fetchedPlaylists = []; // API 调用结果
      commit('SET_PLAYLISTS', fetchedPlaylists);
    },
    selectPlaylist({ commit }, playlistId) {
      commit('TOGGLE_PLAYLIST_SELECTION', playlistId);
    },
    transferSelectedPlaylists({ state }) {
      // 实现选中播放列表的转移逻辑
      console.log('Transferring playlists:', state.selectedPlaylists);
      // 根据业务逻辑和API，这里可能会有更多的操作
    }
  };
  