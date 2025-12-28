package com.garfield100.gendev

import android.os.Bundle
import androidx.fragment.app.Fragment
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.recyclerview.widget.LinearLayoutManager
import com.garfield100.gendev.adapter.CarouselListAdapter
import com.garfield100.gendev.databinding.FragmentFirstBinding
import com.garfield100.gendev.model.Carousel
import com.garfield100.gendev.model.CarouselItem

/**
 * A simple [Fragment] subclass as the default destination in the navigation.
 */
class FirstFragment : Fragment() {

    private var _binding: FragmentFirstBinding? = null

    // This property is only valid between onCreateView and
    // onDestroyView.
    private val binding get() = _binding!!

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {

        _binding = FragmentFirstBinding.inflate(inflater, container, false)
        return binding.root

    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)

        val carousels = getSDUIData()
        binding.carouselsRecyclerview.apply {
            layoutManager = LinearLayoutManager(context)
            adapter = CarouselListAdapter(carousels)
        }
    }

    private fun getSDUIData(): List<Carousel> {
        return listOf(
            Carousel(
                "Carousel 1",
                listOf(
                    CarouselItem("Item 1.1", "Description 1.1", android.R.drawable.star_on),
                    CarouselItem("Item 1.2", "Description 1.2", android.R.drawable.star_off),
                    CarouselItem("Item 1.3", "Description 1.3", android.R.drawable.star_on),
                    CarouselItem("Item 1.4", "Description 1.4", android.R.drawable.star_off),
                )
            ),
            Carousel(
                "Carousel 2",
                listOf(
                    CarouselItem("Item 2.1", "Description 2.1", android.R.drawable.star_on),
                    CarouselItem("Item 2.2", "Description 2.2", android.R.drawable.star_off),
                    CarouselItem("Item 2.3", "Description 2.3", android.R.drawable.star_on),
                )
            ),
            Carousel(
                "Carousel 3",
                listOf(
                    CarouselItem("Item 3.1", "Description 3.1", android.R.drawable.star_on),
                    CarouselItem("Item 3.2", "Description 3.2", android.R.drawable.star_off),
                    CarouselItem("Item 3.3", "Description 3.3", android.R.drawable.star_on),
                    CarouselItem("Item 3.4", "Description 3.4", android.R.drawable.star_off),
                    CarouselItem("Item 3.5", "Description 3.5", android.R.drawable.star_on),
                )
            )
        )
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}